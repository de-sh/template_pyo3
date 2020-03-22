#!/usr/bin/env python
import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

setup_requires = ['setuptools-rust>=0.10.2']

setup(
    name='test-py',
    version='0.1.0',
    description='Example of python with rust',
    rust_extensions=[RustExtension('test_py.test_py', "Cargo.toml", debug=True)],
    install_requires=[],
    setup_requires=setup_requires,
    test_requires=[],
    packages=['test_py'],
    include_package_data=True,
    zip_safe=False,
)