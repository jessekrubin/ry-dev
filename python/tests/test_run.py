import subprocess as sp
import sys
import ry


def test_python_version():
    python_exe = sys.executable
    completed_proc = sp.run([python_exe, "--version"], capture_output=True)
    assert completed_proc.returncode == 0
    print(completed_proc)
    assert "python" in str(completed_proc.stdout).lower()

    thing = ry.run(
        python_exe,
        "--version",
        # capture_output=True,
    )
    print(thing)
    # assert False


def test_binary_output():
    cproc = sp.run(
        [
            # write out some weird binary data
            sys.executable,
            "-c",
            "import sys; sys.stdout.buffer.write(b'\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x09\\x0a\\x0b\\x0c\\x0d\\x0e\\x0f')",
        ],
        capture_output=True,
    )
    assert cproc.returncode == 0
    print(cproc)
    # assert False
