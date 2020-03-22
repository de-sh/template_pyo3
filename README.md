# This is a test of PyO3

Install and use rust nightly:
```
rustup toolchain install nightly
rustup default nightly
```

Initiate and move into Pipenv environment:
```
pipenv shell
```

Install the library:
```
python setup.py install
```

Test the installed library:
```bash
python test/testprint.py
```