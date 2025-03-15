#set -e

# File name (without extension)
filename="main"

rm -r out
mkdir out

# Navigate to the output directory
cd src || mkdir src;

# Compile the LaTeX file
pdflatex -file-line-error -interaction=nonstopmode -synctex=1 -output-format=pdf -output-directory=../out/  ./main.tex
cd ../out || mkdir out;
cp ../src/sources.bib .;
bibtex ./$filename.aux
cd ../src || mkdir src;
pdflatex -file-line-error -interaction=nonstopmode -synctex=1 -output-format=pdf -output-directory=../out/  ./main.tex
pdflatex -file-line-error -interaction=nonstopmode -synctex=1 -output-format=pdf -output-directory=../out/  ./main.tex


# Return to the original directory
cd ..;