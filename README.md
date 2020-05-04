# This is a test of PyO3

Install and use rust nightly:
```
rustup toolchain install nightly
rustup override set nightly
```

Initiate and move into Pipenv environment:
```
pipenv shell
```

Generate a python dynamically linked library from the rust program:
```
python setup.py install
```

Test the installed library:
```bash
python test/testprint.py
```