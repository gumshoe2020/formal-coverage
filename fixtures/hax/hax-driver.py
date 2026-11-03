# Receipt-policy fixture driver. Not invoked by acceptance tests.
import subprocess
import sys
assert sys.argv[1:] == ['extract-fstar']
subprocess.run(['cargo', 'hax', 'into', 'fstar'], check=True)
