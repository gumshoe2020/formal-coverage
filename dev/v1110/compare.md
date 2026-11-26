# v1.11.0 compare: v1.10.0 vs working tree on bertie-tls (bab3c13 staged inputs)

## Proof depth, proven + proven-extracted security runtime SLoC (ghost lines: separate)

| Depth | v1.10.0 lines | v1.10.0 share | v1.11.0 lines | v1.11.0 share |
| --- | ---: | ---: | ---: | ---: |
| safety-only | 3128 | 76.09% | 3797 | 92.36% |
| functional, checked only against own code | 0 | 0.00% | 235 | 5.72% |
| functional | 983 | 23.91% | 79 | 1.92% |

v1.10.0 had no safety-only/functional split of size facts: its `functional` row is every credited line whose postcondition mentioned anything but `result`/`True`.

## Checks

- PASS (a) [separate] 448 units; 0 differ
- PASS (b) [separate] proof ratio 0.14278764290926782, proven 0+4111 of 5078 runtime SLoC; identical to v1.10.0: True
- PASS (d) [separate] depth (3797, 235, 79), proven runtime 4111; recomputed from definitions: True
- PASS (c) [separate] all controls as in PLAN.md
- PASS (e) [separate] 12 unparsed contract(s), each listed in compare.md
- PASS (a) [runtime] 448 units; 0 differ
- PASS (b) [runtime] proof ratio 0.04857846429370943, proven 0+4467 of 5448 runtime SLoC; identical to v1.10.0: True
- PASS (d) [runtime] depth (3995, 327, 145), proven runtime 4467; recomputed from definitions: True
- PASS (c) [runtime] all controls as in PLAN.md
- PASS (e) [runtime] 12 unparsed contract(s), each listed in compare.md

## Controls (separate mode)

| Unit | Expected | Self-consistent | Actual | Match | Postcondition |
| --- | --- | --- | --- | --- | --- |
| tls13formats::client_hello | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData & |
| tls13formats::server_hello | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| tls13formats::server_certificate | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| tls13formats::certificate_verify | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| tls13formats::finished | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| der::decode_ecdsa_signature | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| der::encode_ecdsa_signature | functional | true | functional/value/sc=True | yes | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| der::encode_uint | functional | true | functional/value/sc=True | yes | fun c -> let c:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = c in der_uint_valid (Alloc.Vec.impl_1__as_slice #u8 # |
| der::skip_zeros | functional | false | functional/value/sc=False | yes | fun z -> let z:usize = z in b2t ((i <=. z <: bool) && (z <=. mk_usize 32 <: bool)) /\ zero_prefix buf z /\ b2t |
| der::uint_to_32_bytes | functional | true | functional/value/sc=True | yes | fun res -> let res:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = res in match res |
| tls13utils::check_eq | functional | false | functional/value/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13utils::check_eq1 | functional | false | functional/value/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13utils::check_eq_slice | functional | false | functional/value/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13utils::check_eq_option | functional | false | functional/value/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13utils::check_eq_inner | functional | false | functional/value/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13utils::eq_slice | functional | false | functional/value/sc=False | yes | fun result -> let result:bool = result in ~.result \|\| b1 =. b2) |
| tls13formats::parse_client_hello | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13utils.Byte_types.t_Bytes & Bertie_tls.Tl |
| tls13formats::check_extension | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result (usize & t_Extensions) u8 = result in match result <: Cor |
| tls13formats::check_server_extension | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result (usize & Core_models.Option.t_Option Bertie_tls.Tls13util |
| tls13utils::length_u24_encoded | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| tls13utils::encode_length_u16 | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| tls13utils::bytes2 | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| tls13utils::Bytes::len | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:usize = result in result =. (Alloc.Vec.impl_1__len #u8 #Alloc.Alloc.t_Global self.Ber |
| tls13utils::Bytes::concat | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Rust_primitives.Hax.Int.from_ma |
| tls13utils::check_eq_with_slice | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| tls13crypto::HashAlgorithm::hash_len | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:usize = result in result <=. mk_usize 64) |
| tls13cert::has_oid | safety-only | n/a | safety-only/size-only/sc=False | yes | fun result -> let result:Core_models.Result.t_Result bool u8 = result in match result <: Core_models.Result.t_ |
| tls13record::padlen | safety-only | n/a | safety-only/size-only/sc=False | yes | fun out -> let out:usize = out in out <=. n) (decreases (Rust_primitives.Hax.Int.from_machine n <: Hax_lib.Int |

## Contracts the classifier could not read (reason `unparsed`)

- `Alloc.Collections.Binary_heap.impl_10__pop` (libs/core/Alloc.Collections.Binary_heap.fst:50): `fun temp_0_ -> let (self_e_future: t_BinaryHeap v_T v_A), (res: Core_models.Option.t_Option v_T) = temp_0_ in ((impl_11__len #v_T #v_A self <: usize) >. mk_usize 0 <: bool) =. (Core_models.Option.impl__is_some #v_T res <: bool)) = let (max: Core_models.Option.t_Option v_T):Core_models.Option.t_Option v_T = Core_models.Option.Option_None <: Core_models.Option.t_Option v_T in let index:usize = mk_us`
- `Alloc.Vec.impl_1__remove` (libs/core/Alloc.Vec.fst:124): `fun temp_0_ -> let (self_e_future: t_Vec v_T v_A), (_: v_T) = temp_0_ in (Rust_primitives.Hax.Int.from_machine (impl_1__len #v_T #v_A self_e_future <: usize) <: Hax_lib.Int.t_Int) <= (Rust_primitives.Hax.Int.from_machine (impl_1__len #v_T #v_A self <: usize) <: Hax_lib.Int.t_Int)) unfold`
- `Core_models.Specs.Num.Abs.abs_i16_v` (libs/core/Core_models.Specs.Num.Abs.fst:39): `v (Core_models.Num.impl_i16__abs x) == (if v x < 0 then - (v x) else v x)) = ()`
- `Core_models.Specs.Num.Abs.abs_i32_v` (libs/core/Core_models.Specs.Num.Abs.fst:44): `v (Core_models.Num.impl_i32__abs x) == (if v x < 0 then - (v x) else v x)) = ()`
- `Rust_primitives.Arrays.createi` (libs/rust_primitives/Rust_primitives.Arrays.fsti:25): `(fun res -> (forall i. Seq.index res (v i) == f i))) unfold let map #a #b #p (f:(x:a{p x} -> b)) (s: t_Slice a {forall (i:nat). i < Seq.length s ==> p (Seq.index s i)}): t_Slice b = createi (length s) (fun i -> f (Seq.index s (v i)))`
- `Rust_primitives.Arrays.split` (libs/rust_primitives/Rust_primitives.Arrays.fsti:67): `(fun (x,y) -> x == slice a (sz 0) m /\ y == slice a m (length a) /\ concat #t x y == a)) = let x = Seq.slice a 0 (v m) in let y = Seq.slice a (v m) (Seq.length a) in assert (Seq.equal a (concat x y)); (x,y)`
- `Rust_primitives.BitVectors.bit_vec_to_int_t_array` (libs/rust_primitives/Rust_primitives.BitVectors.fsti:77): `fun r -> (forall i. bit_vec_of_int_t_array r d i == bv i))`
- `Rust_primitives.BitVectors.bit_vec_to_nat_array` (libs/rust_primitives/Rust_primitives.BitVectors.fsti:83): `fun r -> (forall i. bit_vec_of_nat_array r d i == bv i) /\ (forall i. Seq.index r i < pow2 d))`
- `Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range` (libs/rust_primitives/Rust_primitives.Hax.Monomorphized_update_at_Lemmas.fst:19): `( let out = update_at_range s i x in Seq.length out == Seq.length s /\ (forall (j: nat). (j < v i.f_start ==> Seq.index out j == Seq.index s j) /\ (j >= v i.f_start /\ j < v i.f_end ==> Seq.index out j == Seq.index x (j - v i.f_start)) /\ (j >= v i.f_end /\ j < Seq.length out ==> Seq.index out j == Seq.index s j)))) = let out = update_at_range s i x in let aux (j: nat) : Lemma ((j < v i.f_start ==`
- `Rust_primitives.Integers.get_bit_shl` (libs/rust_primitives/Rust_primitives.Integers.fsti:566): `get_bit (x <<! y) i == (if v i < v y then 0 else get_bit x (mk_int (v i - v y)))) [SMTPat (get_bit (x <<! y) i)]`
- `Rust_primitives.Integers.get_bit_shr` (libs/rust_primitives/Rust_primitives.Integers.fsti:573): `get_bit (x >>! y) i == (if v i < bits t - v y then get_bit x (mk_int (v i + v y)) else if signed t then get_bit x (mk_int (bits t - 1)) else 0)) [SMTPat (get_bit (x >>! y) i)]`
- `Rust_primitives.Slice.array_from_fn` (libs/rust_primitives/Rust_primitives.Slice.fsti:26): `(fun a -> forall i. Seq.index a i == f (sz i)))`

## Contracts whose kind or self-consistency changed

| Unit | SLoC | Old | New | Postcondition |
| --- | ---: | --- | --- | --- |
| bertie_tls::tls13formats::parse_client_hello | 138 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13utils.Byte_types.t_Bytes & Bertie_tls.Tl |
| bertie_tls::tls13formats::parse_client_hello | 138 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13utils.Byte_types.t_Bytes & Bertie_tls.Tl |
| bertie_tls::tls13formats::client_hello | 72 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData & |
| bertie_tls::tls13formats::client_hello | 72 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData & |
| bertie_tls::tls13formats::check_extension | 67 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & t_Extensions) u8 = result in match result <: Cor |
| bertie_tls::tls13formats::check_extension | 67 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & t_Extensions) u8 = result in match result <: Cor |
| bertie_tls::tls13formats::server_hello | 39 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::server_hello | 39 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::der::der_roundtrip | 34 | functional/sc=False | functional/value/sc=True | fun temp_0_ -> let _:Prims.unit = temp_0_ in match encode_ecdsa_signature raw <: Core_models.Result.t_Result ( |
| bertie_tls::der::der_roundtrip | 34 | functional/sc=False | functional/value/sc=True | fun temp_0_ -> let _:Prims.unit = temp_0_ in match encode_ecdsa_signature raw <: Core_models.Result.t_Result ( |
| bertie_tls::der::decode_ecdsa_signature | 33 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| bertie_tls::der::decode_ecdsa_signature | 33 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| bertie_tls::tls13formats::certificate_verify | 26 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::certificate_verify | 26 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::check_server_extension | 20 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & Core_models.Option.t_Option Bertie_tls.Tls13util |
| bertie_tls::tls13formats::check_server_extension | 20 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & Core_models.Option.t_Option Bertie_tls.Tls13util |
| bertie_tls::tls13formats::server_certificate | 20 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::server_certificate | 20 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::der::be_value_shift | 19 | functional/sc=False | functional/value/sc=True | fun temp_0_ -> let _:Prims.unit = temp_0_ in (be_value_prefix a n <: Hax_lib.Int.t_Int) = (be_value_prefix b ( |
| bertie_tls::der::be_value_shift | 19 | functional/sc=False | functional/value/sc=True | fun temp_0_ -> let _:Prims.unit = temp_0_ in (be_value_prefix a n <: Hax_lib.Int.t_Int) = (be_value_prefix b ( |
| bertie_tls::tls13utils::check_eq_with_slice | 18 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_eq_with_slice | 18 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::length_u24_encoded | 16 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13utils::length_u24_encoded | 16 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::length | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & usize) u8 = result in match result <: Core_model |
| bertie_tls::tls13cert::length | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (usize & usize) u8 = result in match result <: Core_model |
| bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13utils::length_u16_encoded_slice | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13utils::length_u16_encoded_slice | 15 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::der::encode_ecdsa_signature | 14 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| bertie_tls::der::encode_ecdsa_signature | 14 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = result in  |
| bertie_tls::tls13crypto::Algorithms::ciphersuite | 14 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13crypto::Algorithms::ciphersuite | 14 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::encode_length_u24 | 14 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::encode_length_u24 | 14 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::der::uint_len_unique | 13 | functional/sc=False | safety-only/size-only/sc=False | fun temp_0_ -> let _:Prims.unit = temp_0_ in (Core_models.Slice.impl__len #u8 c1 <: usize) =. (Core_models.Sli |
| bertie_tls::der::uint_len_unique | 13 | functional/sc=False | safety-only/size-only/sc=False | fun temp_0_ -> let _:Prims.unit = temp_0_ in (Core_models.Slice.impl__len #u8 c1 <: usize) =. (Core_models.Sli |
| bertie_tls::tls13cert::long_length | 13 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::long_length | 13 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13utils::encode_length_u16 | 13 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::encode_length_u16 | 13 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::der::uint_to_32_bytes | 12 | functional/sc=False | functional/value/sc=True | fun res -> let res:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = res in match res |
| bertie_tls::der::uint_to_32_bytes | 12 | functional/sc=False | functional/value/sc=True | fun res -> let res:Core_models.Result.t_Result (Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global) u8 = res in match res |
| bertie_tls::tls13cert::has_oid | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result bool u8 = result in match result <: Core_models.Result.t_ |
| bertie_tls::tls13cert::has_oid | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result bool u8 = result in match result <: Core_models.Result.t_ |
| bertie_tls::tls13utils::encode_length_u8 | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::encode_length_u8 | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::length_u8_encoded | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13utils::length_u8_encoded | 12 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::check_tag | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13cert::check_tag | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13cert::length_length | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::length_length | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_version_number | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_version_number | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::short_length | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::short_length | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13crypto::Algorithms::supported_group | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13crypto::Algorithms::supported_group | 11 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::der::pow256 | 10 | functional/sc=False | safety-only/size-only/sc=False | fun p -> let p:Hax_lib.Int.t_Int = p in p >= (Rust_primitives.Hax.Int.from_machine (mk_u8 1) <: Hax_lib.Int.t_ |
| bertie_tls::der::pow256 | 10 | functional/sc=False | safety-only/size-only/sc=False | fun p -> let p:Hax_lib.Int.t_Int = p in p >= (Rust_primitives.Hax.Int.from_machine (mk_u8 1) <: Hax_lib.Int.t_ |
| bertie_tls::tls13formats::finished | 10 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::finished | 10 | functional/sc=False | functional/value/sc=True | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13formats.Handshake_data.t_HandshakeData u8 |
| bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message | 10 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (t_HandshakeData & t_HandshakeData) u8 = result in match  |
| bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message | 10 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result (t_HandshakeData & t_HandshakeData) u8 = result in match  |
| bertie_tls::tls13utils::Bytes::prefix | 10 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) > |
| bertie_tls::tls13utils::Bytes::prefix | 10 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) > |
| bertie_tls::der::encode_uint | 9 | functional/sc=False | functional/value/sc=True | fun c -> let c:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = c in der_uint_valid (Alloc.Vec.impl_1__as_slice #u8 # |
| bertie_tls::der::encode_uint | 9 | functional/sc=False | functional/value/sc=True | fun c -> let c:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = c in der_uint_valid (Alloc.Vec.impl_1__as_slice #u8 # |
| bertie_tls::tls13formats::handshake_data::from_bytes_inner | 9 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13formats::handshake_data::from_bytes_inner | 9 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13utils::concat_inner | 9 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Rust_primitives.Hax.Int.from_ma |
| bertie_tls::tls13utils::concat_inner | 9 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Rust_primitives.Hax.Int.from_ma |
| bertie_tls::tls13crypto::HashAlgorithm::hash_len | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result <=. mk_usize 64) |
| bertie_tls::tls13crypto::HashAlgorithm::hash_len | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result <=. mk_usize 64) |
| bertie_tls::tls13formats::build_server_name | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13formats::build_server_name | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::Bytes::declassify | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = result in (Alloc.Vec.impl_1__len #u8 #Alloc |
| bertie_tls::tls13utils::Bytes::declassify | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Alloc.Vec.t_Vec u8 Alloc.Alloc.t_Global = result in (Alloc.Vec.impl_1__len #u8 #Alloc |
| bertie_tls::tls13utils::check_length_encoding_u16_slice | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u16_slice | 8 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13cert::advance | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::advance | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13crypto::Algorithms::check | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13crypto::Algorithms::check | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13crypto::Algorithms::signature_algorithm | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13crypto::Algorithms::signature_algorithm | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13record::padlen | 7 | functional/sc=False | safety-only/size-only/sc=False | fun out -> let out:usize = out in out <=. n) (decreases (Rust_primitives.Hax.Int.from_machine n <: Hax_lib.Int |
| bertie_tls::tls13record::padlen | 7 | functional/sc=False | safety-only/size-only/sc=False | fun out -> let out:usize = out in out <=. n) (decreases (Rust_primitives.Hax.Int.from_machine n <: Hax_lib.Int |
| bertie_tls::tls13utils::check_length_encoding_u24 | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u24 | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u8_slice | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u8_slice | 7 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13cert::read_header | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_header | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_tagged | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_tagged | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes | 6 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result t_HandshakeData u8 = result in match result <: Core_model |
| bertie_tls::tls13formats::handshake_record | 5 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13formats::handshake_record | 5 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13cert::read_slice | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13cert::read_slice | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13formats::handshake_data::HandshakeData::len | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result =. (Bertie_tls.Tls13utils.impl_17__len self._0 <: usize)) |
| bertie_tls::tls13formats::handshake_data::HandshakeData::len | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result =. (Bertie_tls.Tls13utils.impl_17__len self._0 <: usize)) |
| bertie_tls::tls13formats::handshake_data::to_bytes_inner | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Bertie_tls.Tls13utils.impl_17__ |
| bertie_tls::tls13formats::handshake_data::to_bytes_inner | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Bertie_tls.Tls13utils.impl_17__ |
| bertie_tls::tls13utils::Bytes::as_raw | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:t_Slice u8 = result in (Core_models.Slice.impl__len #u8 result <: usize) =. (Alloc.Ve |
| bertie_tls::tls13utils::Bytes::as_raw | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:t_Slice u8 = result in (Core_models.Slice.impl__len #u8 result <: usize) =. (Alloc.Ve |
| bertie_tls::tls13utils::Bytes::zeroes | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::Bytes::zeroes | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::check_length_encoding_u8 | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u8 | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::length_u16_encoded | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13utils::length_u16_encoded | 4 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::der::two_pow_256 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun bound -> let bound:Hax_lib.Int.t_Int = bound in bound > (Rust_primitives.Hax.Int.from_machine (mk_u8 0) <: |
| bertie_tls::der::two_pow_256 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun bound -> let bound:Hax_lib.Int.t_Int = bound in bound > (Rust_primitives.Hax.Int.from_machine (mk_u8 0) <: |
| bertie_tls::tls13cert::read_octet_header | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_octet_header | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_sequence_header | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::read_sequence_header | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_integer | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_integer | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_sequence | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13cert::skip_sequence | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result usize u8 = result in match result <: Core_models.Result.t |
| bertie_tls::tls13formats::handshake_data::HandshakeData::to_bytes | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Bertie_tls.Tls13utils.impl_17__ |
| bertie_tls::tls13formats::handshake_data::HandshakeData::to_bytes | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Bertie_tls.Tls13utils.impl_17__ |
| bertie_tls::tls13formats::supported_versions | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13formats::supported_versions | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Bertie_tls.Tls13utils.Byte_types.t_Bytes u8 = result in m |
| bertie_tls::tls13utils::Bytes::append | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::append | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::concat | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Rust_primitives.Hax.Int.from_ma |
| bertie_tls::tls13utils::Bytes::concat | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Rust_primitives.Hax.Int.from_ma |
| bertie_tls::tls13utils::Bytes::extend_from_slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::extend_from_slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::from_slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::Bytes::from_slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::Bytes::len | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result =. (Alloc.Vec.impl_1__len #u8 #Alloc.Alloc.t_Global self.Ber |
| bertie_tls::tls13utils::Bytes::len | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:usize = result in result =. (Alloc.Vec.impl_1__len #u8 #Alloc.Alloc.t_Global self.Ber |
| bertie_tls::tls13utils::Bytes::new_alloc | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::Bytes::new_alloc | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::Bytes::push | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::push | 3 | functional/sc=False | safety-only/size-only/sc=False | fun self_e_future -> let self_e_future:Bertie_tls.Tls13utils.Byte_types.t_Bytes = self_e_future in (Rust_primi |
| bertie_tls::tls13utils::Bytes::slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::Bytes::slice | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (Alloc.Vec.impl_1__len #u8 #Allo |
| bertie_tls::tls13utils::bytes | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::bytes | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::bytes1 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::bytes1 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::bytes2 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::bytes2 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Bertie_tls.Tls13utils.Byte_types.t_Bytes = result in (impl_17__len result <: usize) = |
| bertie_tls::tls13utils::check_length_encoding_u16 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| bertie_tls::tls13utils::check_length_encoding_u16 | 3 | functional/sc=False | safety-only/size-only/sc=False | fun result -> let result:Core_models.Result.t_Result Prims.unit u8 = result in match result <: Core_models.Res |
| Alloc.Collections.Binary_heap.impl_10__pop | 0 | functional/sc=False | safety-only/unparsed/sc=False | fun temp_0_ -> let (self_e_future: t_BinaryHeap v_T v_A), (res: Core_models.Option.t_Option v_T) = temp_0_ in  |
| Alloc.Vec.impl_1__remove | 0 | functional/sc=False | safety-only/unparsed/sc=False | fun temp_0_ -> let (self_e_future: t_Vec v_T v_A), (_: v_T) = temp_0_ in (Rust_primitives.Hax.Int.from_machine |
| Core_models.Bundle.impl__is_some | 0 | functional/sc=True | functional/value/sc=False | fun res -> let res:bool = res in b2t res ==> Option_Some? self) = match self <: t_Option v_T with \| Option_Som |
| Core_models.Specs.Num.Abs.abs_i16_v | 0 | functional/sc=False | safety-only/unparsed/sc=False | v (Core_models.Num.impl_i16__abs x) == (if v x < 0 then - (v x) else v x)) = () |
| Core_models.Specs.Num.Abs.abs_i32_v | 0 | functional/sc=False | safety-only/unparsed/sc=False | v (Core_models.Num.impl_i32__abs x) == (if v x < 0 then - (v x) else v x)) = () |
| Rust_primitives.Arrays.createi | 0 | functional/sc=False | safety-only/unparsed/sc=False | (fun res -> (forall i. Seq.index res (v i) == f i))) unfold let map #a #b #p (f:(x:a{p x} -> b)) (s: t_Slice a |
| Rust_primitives.Arrays.split | 0 | functional/sc=False | safety-only/unparsed/sc=False | (fun (x,y) -> x == slice a (sz 0) m /\ y == slice a m (length a) /\ concat #t x y == a)) = let x = Seq.slice a |
| Rust_primitives.BitVectors.bit_vec_to_int_t_array | 0 | functional/sc=False | safety-only/unparsed/sc=False | fun r -> (forall i. bit_vec_of_int_t_array r d i == bv i)) |
| Rust_primitives.BitVectors.bit_vec_to_nat_array | 0 | functional/sc=False | safety-only/unparsed/sc=False | fun r -> (forall i. bit_vec_of_nat_array r d i == bv i) /\ (forall i. Seq.index r i < pow2 d)) |
| Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range | 0 | functional/sc=False | safety-only/unparsed/sc=False | ( let out = update_at_range s i x in Seq.length out == Seq.length s /\ (forall (j: nat). (j < v i.f_start ==>  |
| Rust_primitives.Integers.get_bit_cast | 0 | functional/sc=True | functional/value/sc=False | get_bit (cast_mod #t #u x) nth == get_bit x nth) [SMTPat (get_bit (cast_mod #t #u x) nth)] |
| Rust_primitives.Integers.get_bit_cast_extend | 0 | functional/sc=True | functional/value/sc=False | get_bit (cast_mod #t #u x) nth == 0) [SMTPat (get_bit (cast_mod #t #u x) nth)] |
| Rust_primitives.Integers.get_bit_shl | 0 | functional/sc=True | safety-only/unparsed/sc=False | get_bit (x <<! y) i == (if v i < v y then 0 else get_bit x (mk_int (v i - v y)))) [SMTPat (get_bit (x <<! y) i |
| Rust_primitives.Integers.get_bit_shr | 0 | functional/sc=True | safety-only/unparsed/sc=False | get_bit (x >>! y) i == (if v i < bits t - v y then get_bit x (mk_int (v i + v y)) else if signed t then get_bi |
| Rust_primitives.Integers.logor_disjoint | 0 | functional/sc=True | functional/value/sc=False | v (logor a b) == v a + v b) |
| Rust_primitives.Slice.array_from_fn | 0 | functional/sc=False | safety-only/unparsed/sc=False | (fun a -> forall i. Seq.index a i == f (sz i))) |
