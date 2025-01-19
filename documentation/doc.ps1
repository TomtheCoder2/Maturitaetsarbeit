$ErrorActionPreference = "Stop"

# File name (without extension)
$filename = "poster/poster"

# Navigate to the output directory
cd out

# Compile the LaTeX file
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/ --include-directory=../src/poster
bibtex ./$filename.aux --include-directory=../src/
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/ --include-directory=../src/poster
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/ --include-directory=../src/poster
pdflatex ../src/$filename.tex --interaction=nonstopmode --output-directory=./ --include-directory=../src/ --include-directory=../src/poster

# Return to the original directory
cd ..
