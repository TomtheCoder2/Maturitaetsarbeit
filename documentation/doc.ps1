$ErrorActionPreference = "Stop";
cd out;
pdflatex ../src/main.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
bibtex .\main.aux --include-directory=../src/;
pdflatex ../src/main.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
pdflatex ../src/main.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
pdflatex ../src/main.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
cd ..;