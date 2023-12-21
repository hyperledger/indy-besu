"""Module setup."""

import os
import runpy
from setuptools import find_packages, setup

PACKAGE_NAME = "indy2_vdr"
version_meta = runpy.run_path("./{}/version.py".format(PACKAGE_NAME))
VERSION = version_meta["__version__"]

with open(os.path.abspath("./README.md"), "r") as fh:
    long_description = fh.read()

if __name__ == "__main__":
    setup(
        name=PACKAGE_NAME,
        version=VERSION,
        author="DSR Corporation",
        author_email="artem.ivanov@dsr-corporation.com",
        long_description=long_description,
        long_description_content_type="text/markdown",
        packages=find_packages(),
        include_package_data=True,
        package_data={
            "": [
                "indy2_vdr.dll",
                "libindy2_vdr.dylib",
                "libindy2_vdr.so",
            ]
        },
        python_requires=">=3.6.3",
        classifiers=[
            "Programming Language :: Python :: 3",
            "License :: OSI Approved :: Apache Software License",
            "Operating System :: OS Independent",
        ],
    )
