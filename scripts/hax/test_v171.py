import copy
import tempfile
import unittest
from pathlib import Path

import lock_audit_v2 as audit
import v171_numbers as numbers
import v171_rootlock as rootlock
import v171_workspace as workspace

GIT = "git+https://github.com/gumshoe2020/bertie?rev=abc#abc"
REG = "registry+https://github.com/rust-lang/crates.io-index"


def pkg(name, version="1.0.0", deps=(), source=REG, checksum=None):
    p = {"name": name, "version": version, "dependencies": list(deps)}
    if source:
        p["source"] = source
        p["checksum"] = checksum or "sum-" + name + version
    return p


def shipped_lock():
    return [
        pkg("bertie", "0.1.0", ["rand", "uuid"], source=GIT),
        pkg("rand", deps=["rand_core 0.9.0"]),
        pkg("rand_core", "0.9.0", ["getrandom"]),
        pkg("uuid", deps=["getrandom"]),
        pkg("getrandom", deps=["rand_core 0.10.0"]),
        pkg("rand_core", "0.10.0"),
        pkg("scratch-server", deps=["bertie", "rand_core 0.10.0", "other"], source=None),
        pkg("other", deps=[]),
    ]


def path_bertie(deps=("rand", "uuid", "dhat")):
    return {"name": "bertie", "version": "0.1.0", "dependencies": list(deps)}


def current_lock():
    lock = [p for p in shipped_lock() if p["name"] != "bertie"]
    lock.append(path_bertie())
    lock.append(pkg("dhat"))
    return lock


CLOSURE = {("rand", "1.0.0"), ("rand_core", "0.9.0"), ("uuid", "1.0.0"), ("getrandom", "1.0.0"), ("rand_core", "0.10.0")}
DEV_ONLY = {("dhat", "1.0.0")}
TREE_OK = {"ok": True, "problems": []}


def run(current, closure=CLOSURE, dev_only=DEV_ONLY, tree=TREE_OK):
    return audit.audit(shipped_lock(), current, closure, dev_only, tree)


def kinds(result):
    return sorted(f["kind"] for f in result["blocking"])


def replace(lock, name, current_version, **changes):
    lock = copy.deepcopy(lock)
    for p in lock:
        if p["name"] == name and p["version"] == current_version:
            p.update(changes)
    return lock


class LockAuditV2(unittest.TestCase):
    def test_exact_lock_passes_with_bertie_source_change_and_dev_addition_listed(self):
        result = run(current_lock())
        self.assertEqual(result["verdict"], "pass")
        listed = {(a["kind"], a["package"]) for a in result["additions"]}
        self.assertIn(("package", "dhat 1.0.0"), listed)
        self.assertIn(("bertie-dev-edge", "bertie"), listed)

    def test_version_change_is_rejected(self):
        lock = replace(current_lock(), "rand", "1.0.0", version="1.0.1")
        self.assertIn("version-change", kinds(run(lock)))

    def test_source_change_is_rejected(self):
        lock = replace(current_lock(), "other", "1.0.0", source="git+https://example.invalid/x")
        self.assertIn("source-change", kinds(run(lock)))

    def test_checksum_change_is_rejected(self):
        lock = replace(current_lock(), "other", "1.0.0", checksum="different")
        self.assertIn("checksum-change", kinds(run(lock)))

    def test_removed_package_is_rejected_even_outside_closure(self):
        lock = [p for p in current_lock() if p["name"] != "other"]
        lock = replace(lock, "scratch-server", "1.0.0", dependencies=["bertie", "rand_core 0.10.0"])
        self.assertIn("removed", kinds(run(lock)))

    def test_closure_internal_addition_is_rejected(self):
        lock = current_lock() + [pkg("extra")]
        closure = CLOSURE | {("extra", "1.0.0")}
        self.assertIn("closure-internal-addition", kinds(run(lock, closure=closure)))

    def test_dropped_edge_inside_closure_is_rejected(self):
        lock = replace(current_lock(), "getrandom", "1.0.0", dependencies=[])
        result = run(lock)
        self.assertEqual(kinds(result), ["dropped-edge"])
        self.assertEqual(result["blocking"][0]["detail"], "rand_core 0.10.0")

    def test_dropped_edge_outside_closure_is_rejected(self):
        lock = replace(current_lock(), "scratch-server", "1.0.0", dependencies=["bertie", "other"])
        self.assertEqual(kinds(run(lock)), ["dropped-edge"])

    def test_added_edge_inside_closure_is_rejected_but_outside_is_listed(self):
        inside = replace(current_lock(), "rand", "1.0.0", dependencies=["rand_core 0.9.0", "other"])
        self.assertIn("closure-internal-edge-addition", kinds(run(inside)))
        outside = replace(current_lock(), "other", "1.0.0", dependencies=["dhat"])
        result = run(outside)
        self.assertEqual(result["verdict"], "pass")
        self.assertIn(("edge-outside-closure", "other 1.0.0"), {(a["kind"], a["package"]) for a in result["additions"]})

    def test_bertie_edge_to_non_dev_target_is_rejected(self):
        lock = [p for p in current_lock() if p["name"] != "bertie"] + [path_bertie(("rand", "uuid", "dhat", "other"))]
        self.assertEqual(kinds(run(lock)), ["closure-internal-edge-addition"])

    def test_bertie_must_become_a_path_package(self):
        lock = [p for p in current_lock() if p["name"] != "bertie"] + [pkg("bertie", "0.1.0", ["rand", "uuid", "dhat"], source=GIT)]
        self.assertIn("bertie-source-not-path", kinds(run(lock)))

    def test_reference_spelling_change_from_a_second_version_is_not_an_edge_change(self):
        lock = replace(current_lock(), "other", "1.0.0", dependencies=["dhat 1.0.0"]) + [pkg("dhat", "2.0.0")]
        lock = replace(lock, "bertie", "0.1.0", dependencies=["rand", "uuid", "dhat 1.0.0"])
        shipped = replace(shipped_lock(), "other", "1.0.0", dependencies=["dhat"]) + [pkg("dhat", "1.0.0")]
        result = audit.audit(shipped, lock, CLOSURE, {("dhat", "1.0.0"), ("dhat", "2.0.0")}, TREE_OK)
        self.assertEqual(result["verdict"], "pass", result["blocking"])

    def test_unresolvable_reference_blocks(self):
        lock = replace(current_lock(), "other", "1.0.0", dependencies=["nonexistent"])
        self.assertEqual(kinds(run(lock)), ["unresolvable-reference"])

    def test_tree_hash_mismatch_blocks(self):
        tree = audit.check_tree({"a.rs": "1"}, {"a.rs": "2"}, {})
        self.assertFalse(tree["ok"])
        self.assertIn("tree-hash-mismatch", kinds(run(current_lock(), tree=tree)))


class TreeCheck(unittest.TestCase):
    def test_unrecorded_extra_missing_and_edited_files_are_problems(self):
        archive = {"a": "1", "b": "2", "c": "3"}
        self.assertTrue(audit.check_tree(archive, dict(archive), {})["ok"])
        self.assertFalse(audit.check_tree(archive, {"a": "1", "b": "2"}, {})["ok"])
        self.assertFalse(audit.check_tree(archive, {**archive, "d": "4"}, {})["ok"])
        self.assertFalse(audit.check_tree(archive, {**archive, "c": "9"}, {})["ok"])
        self.assertTrue(audit.check_tree(archive, {**archive, "c": "9"}, {"c": {"before": "3", "after": "9"}})["ok"])
        self.assertFalse(audit.check_tree(archive, {**archive, "c": "8"}, {"c": {"before": "3", "after": "9"}})["ok"])


class ParsingHelpers(unittest.TestCase):
    def test_cargo_tree_lines_parse_to_name_version(self):
        text = "bertie v0.1.0-pre.2 (/ws/bertie)\nrand v0.9.5\nrand_core v0.9.5 (*)\n"
        self.assertEqual(audit.closure_from_cargo_tree(text),
                         {("bertie", "0.1.0-pre.2"), ("rand", "0.9.5"), ("rand_core", "0.9.5")})

    def test_dev_only_targets_ignore_normal_edges(self):
        ids = {"b": ("bertie", "1"), "d": ("dhat", "1"), "r": ("rand", "1")}
        metadata = {
            "packages": [{"id": k, "name": v[0], "version": v[1]} for k, v in ids.items()],
            "resolve": {"nodes": [{"id": "b", "deps": [
                {"pkg": "d", "dep_kinds": [{"kind": "dev"}]},
                {"pkg": "r", "dep_kinds": [{"kind": None}, {"kind": "dev"}]}]}]},
        }
        self.assertEqual(audit.dev_only_targets_from_metadata(metadata), {("dhat", "1")})


MEMBER_MANIFEST = '[package]\nname = "x"\n\n[workspace]\nmembers = ["crates/a", "crates/b"]\n\n[dependencies]\n'
BERTIE_MANIFEST = '[package]\nname = "bertie"\n\n[workspace]\nmembers = [\n    ".",\n    "record",\n]\n\ndefault-members = [\n    ".",\n]\n\n[lints.rust]\nx = 1\n'


class WorkspaceEdits(unittest.TestCase):
    def test_member_and_patch_are_the_only_scratch_server_edits(self):
        edited = workspace.add_member_and_patch(MEMBER_MANIFEST)
        self.assertIn('members = ["crates/a", "crates/b", "bertie"]', edited)
        self.assertTrue(edited.endswith('[patch."https://github.com/gumshoe2020/bertie"]\nbertie = { path = "bertie" }\n'))
        self.assertEqual(edited.replace(', "bertie"', "").split("\n[patch")[0], MEMBER_MANIFEST)

    def test_bertie_workspace_table_removed_including_default_members(self):
        edited = workspace.drop_bertie_workspace_table(BERTIE_MANIFEST)
        self.assertEqual(edited, '[package]\nname = "bertie"\n\n[lints.rust]\nx = 1\n')

    def test_unexpected_manifest_shapes_are_refused(self):
        with self.assertRaises(ValueError):
            workspace.drop_bertie_workspace_table('[package]\nname = "b"\n')
        with self.assertRaises(ValueError):
            workspace.add_member_and_patch(MEMBER_MANIFEST + '[patch.crates-io]\n')

    def test_built_workspace_matches_archives_except_recorded_edits(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            (tmp / "s").mkdir(); (tmp / "b").mkdir()
            (tmp / "s/Cargo.toml").write_text(MEMBER_MANIFEST)
            (tmp / "s/src.rs").write_text("fn a() {}")
            (tmp / "s/Cargo.lock").write_text("version = 4\n")
            (tmp / "b/Cargo.toml").write_text(BERTIE_MANIFEST)
            (tmp / "b/lib.rs").write_text("fn b() {}")
            ws = tmp / "out/ws"
            ws.parent.mkdir()
            self.assertEqual(workspace.cmd_build(str(tmp / "s"), str(tmp / "b"), str(ws)), 0)
            (ws / "bertie/lib.rs").write_text("fn b() { panic!() }")
            self.assertEqual(workspace.cmd_check(str(ws)), 1)


class RootLock(unittest.TestCase):
    def test_pruned_derived_lock_passes_and_lists_removals(self):
        derived = [p for p in shipped_lock() if p["name"] != "other"]
        result = rootlock.compare(shipped_lock(), derived)
        self.assertTrue(result["ok"])
        self.assertIn("other 1.0.0", result["only_workspace"])

    def test_source_or_checksum_change_blocks(self):
        derived = replace(shipped_lock(), "other", "1.0.0", source="git+https://example.invalid/x")
        self.assertIn("identity change: other 1.0.0", rootlock.compare(shipped_lock(), derived)["problems"])

    def test_replaced_version_blocks_but_added_new_package_does_not(self):
        drifted = [p for p in shipped_lock() if p["name"] != "other"] + [pkg("other", "2.0.0")]
        self.assertFalse(rootlock.compare(shipped_lock(), drifted)["ok"])
        added = shipped_lock() + [pkg("brand_new")]
        self.assertTrue(rootlock.compare(shipped_lock(), added)["ok"])

    def test_dependency_edge_loss_is_listed_not_blocking(self):
        derived = replace(shipped_lock(), "uuid", "1.0.0", dependencies=[])
        result = rootlock.compare(shipped_lock(), derived)
        self.assertTrue(result["ok"])
        self.assertEqual(result["edge_differences"][0]["only_workspace"], ["getrandom 1.0.0"])


class ScopeClassification(unittest.TestCase):
    def reason(self, source, line, file="src/a.rs", unit_id="bertie::a::f"):
        return numbers.scope_reason({"id": unit_id, "file": file, "line": line}, source.split("\n"))

    def test_cfg_evaluation_under_proof_and_shipped_configs(self):
        self.assertTrue(numbers.evaluate('all(feature = "api", feature = "std")', numbers.SHIPPED))
        self.assertFalse(numbers.evaluate('all(feature = "api", feature = "std")', numbers.PROOF))
        self.assertTrue(numbers.evaluate('not(feature = "secret_integers")', numbers.PROOF))
        self.assertTrue(numbers.evaluate("hax", numbers.PROOF))
        self.assertFalse(numbers.evaluate("hax", numbers.SHIPPED))

    def test_unit_cfg_and_enclosing_impl_cfg_are_both_read(self):
        own = '#[cfg(feature = "hax-pv")]\nfn f() {}'
        self.assertEqual(self.reason(own, 2), "feature-gated (inactive in both builds)")
        impl = '#[cfg(feature = "api")]\nimpl X {\n    fn f() {}\n}'
        self.assertEqual(self.reason(impl, 3), "feature-gated (shipped-only)")
        self.assertIsNone(self.reason('#[cfg(feature = "std")]\nfn f() {}', 2))

    def test_stream_and_non_hax_take_precedence(self):
        self.assertEqual(self.reason("fn f() {}", 1, file="src/stream/x.rs"), "stream")
        self.assertEqual(self.reason("fn f() {}", 1, unit_id="bertie::tls13utils::non_hax::f"), "non_hax")


if __name__ == "__main__":
    unittest.main()
