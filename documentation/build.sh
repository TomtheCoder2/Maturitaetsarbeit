cd out;
pdflatex ../src/diff.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
bibtex .\diff.aux --include-directory=../src/;
pdflatex ../src/diff.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
pdflatex ../src/diff.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
pdflatex ../src/diff.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/;
cd ..;