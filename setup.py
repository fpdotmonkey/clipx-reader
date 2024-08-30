# taken from https://stackoverflow.com/a/28408331
import os
import shutil
import subprocess
import sys

from distutils.command.build_py import build_py as _build_py
from distutils.command.clean import clean as _clean
from distutils.debug import DEBUG
from distutils.dist import Distribution
from distutils.spawn import find_executable
from setuptools import setup

PROTO_FILES = ["./monitor.proto"]
CLEANUP_SUFFIXES = [
    # filepath suffixes of files to remove on "clean" subcommand
    "_pb2.py",
    ".pyc",
    ".so",
    ".o",
    "dependency_links.txt",
    "entry_points.txt",
    "PKG-INFO",
    "top_level.txt",
    "SOURCES.txt",
    ".coverage",
    "protobuf/compiler/__init__.py",
]
GEN_DIRECTORY = "./client/gen/"

CLEANUP_DIRECTORIES = [  # subdirectories to remove on "clean" subcommand
    # 'build'  # Note: the build subdirectory is removed if --all is set.
    "html-coverage",
    GEN_DIRECTORY,
]

if "PROTOC" in os.environ and os.path.exists(os.environ["PROTOC"]):
    protoc = os.environ["PROTOC"]
else:
    protoc = shutil.which("protoc")


def generate_proto(source):
    """Invoke Protocol Compiler to generate python from given source .proto."""
    if not os.path.exists(source):
        sys.stderr.write("Can't find required file: %s\n" % source)
        sys.exit(1)

    output = source.replace(".proto", "_pb2.py")
    output = "./client/gen/" + output
    if not os.path.exists(GEN_DIRECTORY):
        os.makedirs(GEN_DIRECTORY)
    if not os.path.exists(output) or (
        os.path.getmtime(source) > os.path.getmtime(output)
    ):
        print("Generating %s" % output)

        if protoc is None:
            sys.stderr.write(
                "protoc not found. Is protobuf-compiler installed? \n"
                "Alternatively, you can point the PROTOC environment variable at a "
                "local version."
            )
            sys.exit(1)

        protoc_command = [
            protoc,
            "-I.",
            "-I./3rd/protobufs/include/",
            "--python_out=./client/gen/",
            source,
        ]
        if subprocess.call(protoc_command) != 0:
            sys.exit(1)


class MyDistribution(Distribution):
    # Helper class to add the ability to set a few extra arguments
    # in setup():
    # protofiles : Protocol buffer definitions that need compiling
    # cleansuffixes : Filename suffixes (might be full names) to remove when
    #                   "clean" is called
    # cleandirectories : Directories to remove during cleanup
    # Also, the class sets the clean, build_py, test and nosetests cmdclass
    # options to defaults that compile protobufs, implement test as nosetests
    # and enables the nosetests command as well as using our cleanup class.

    def __init__(self, attrs=None):
        self.protofiles = []  # default to no protobuf files
        self.cleansuffixes = [
            "_pb2.py",
            ".pyc",
        ]  # default to clean generated files
        self.cleandirectories = [
            "html-coverage"
        ]  # clean out coverage directory
        cmdclass = attrs.get("cmdclass")
        if not cmdclass:
            cmdclass = {}
        # These should actually modify attrs['cmdclass'], as we assigned the
        # mutable dict to cmdclass without copying it.
        if "build_py" not in cmdclass:
            cmdclass["build_py"] = MyBuildPy
        if "clean" not in cmdclass:
            cmdclass["clean"] = MyClean
        attrs["cmdclass"] = cmdclass
        # call parent __init__ in old style class
        Distribution.__init__(self, attrs)


class MyClean(_clean):

    def run(self):
        try:
            cleandirectories = self.distribution.cleandirectories
        except AttributeError:
            sys.stderr.write(
                "Error: cleandirectories not defined. MyDistribution not used?"
            )
            sys.exit(1)
        try:
            cleansuffixes = self.distribution.cleansuffixes
        except AttributeError:
            sys.stderr.write(
                "Error: cleansuffixes not defined. MyDistribution not used?"
            )
            sys.exit(1)
        # Remove build and html-coverage directories if they exist
        for directory in cleandirectories:
            if os.path.exists(directory):
                if DEBUG:
                    print('Removing directory: "{}"'.format(directory))
                shutil.rmtree(directory)
        # Delete generated files in code tree.
        for dirpath, _, filenames in os.walk("."):
            for filename in filenames:
                filepath = os.path.join(dirpath, filename)
                for i in cleansuffixes:
                    if filepath.endswith(i):
                        if DEBUG:
                            print('Removing file: "{}"'.format(filepath))
                        os.remove(filepath)
        # _clean is an old-style class, so super() doesn't work
        _clean.run(self)


class MyBuildPy(_build_py):

    def run(self):
        try:
            protofiles = self.distribution.protofiles
        except AttributeError:
            sys.stderr.write(
                "Error: protofiles not defined. MyDistribution not used?"
            )
            sys.exit(1)
        for proto in protofiles:
            generate_proto(proto)
        # _build_py is an old-style class, so super() doesn't work
        _build_py.run(self)


setup(
    # MyDistribution automatically enables several extensions, including
    # the compilation of protobuf files.
    distclass=MyDistribution,
    protofiles=PROTO_FILES,
    cleansuffixes=CLEANUP_SUFFIXES,
    cleandirectories=CLEANUP_DIRECTORIES,
)
