$ErrorActionPreference = "Stop"

# File name (without extension)
$filename = "plagiat_version"

# Navigate to the output directory
cd out

# Compile the LaTeX file
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/
bibtex ./$filename.aux --include-directory=../src/
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/

# Return to the original directory
cd ..
