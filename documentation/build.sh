set -e

# File name (without extension)
filename="main"

# Navigate to the output directory
cd out;

# Compile the LaTeX file
pdflatex ../src/$filename.tex -interaction=nonstopmode -output-directory=./ -include-directory=../src/ --include-directory=../src/
bibtex ./$filename.aux --include-directory=../src/
pdflatex ../src/$filename.tex -interaction=nonstopmode -output-directory=./ -include-directory=../src/ --include-directory=../src/
pdflatex ../src/$filename.tex -interaction=nonstopmode -output-directory=./ -include-directory=../src/ --include-directory=../src/
pdflatex ../src/$filename.tex -interaction=nonstopmode -output-directory=./ -include-directory=../src/ --include-directory=../src/

# Return to the original directory
cd ..;