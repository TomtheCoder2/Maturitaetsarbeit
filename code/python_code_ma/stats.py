import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from scipy.stats import binom, norm

# Input data
data = [
22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,

]

# Basic statistics
mean = np.mean(data)
median = np.median(data)
std_dev = np.std(data)

print(f"Mean: {mean}")
print(f"Median: {median}")
print(f"Standard Deviation: {std_dev}")

# Check for binomial-like distribution
data_min = min(data)
data_max = max(data)
unique, counts = np.unique(data, return_counts=True)
prob = counts / len(data)

# Fitting a binomial distribution (assuming estimated parameters)
n = int(data_max - data_min)  # Approximate number of trials
p = mean / data_max          # Approximate probability of success

# Binomial distribution fit
x_binom = range(data_min, data_max + 1)
binom_pmf = [binom.pmf(k, n, p) for k in x_binom]

# Normal distribution fit (for comparison)
x_norm = np.linspace(data_min, data_max, 500)
norm_pdf = norm.pdf(x_norm, mean, std_dev)

# Plotting
plt.figure(figsize=(16, 10))

# Bar plot of data frequencies
plt.subplot(2, 2, 1)
sns.barplot(x=unique, y=counts, color="blue")
plt.title("Bar Plot of Data Frequencies")
plt.xlabel("Value")
plt.ylabel("Frequency")

# Line plot of data frequencies
plt.subplot(2, 2, 2)
plt.plot(unique, counts, marker="o", linestyle="-", color="green", label="Data")
plt.plot(x_binom, np.array(binom_pmf) * len(data), linestyle="--", color="red", label="Binomial Fit")
plt.legend()
plt.title("Line Plot with Binomial Fit")
plt.xlabel("Value")
plt.ylabel("Frequency")

# Histogram of data
plt.subplot(2, 2, 3)
sns.histplot(data, bins=10, kde=False, color="purple")
plt.title("Histogram of Data")
plt.xlabel("Value")
plt.ylabel("Frequency")

# Normal distribution fit
plt.subplot(2, 2, 4)
plt.hist(data, bins=10, density=True, alpha=0.6, color="gray", label="Data Histogram")
plt.plot(x_norm, norm_pdf, color="orange", label="Normal Fit")
plt.legend()
plt.title("Normal Fit vs Data")
plt.xlabel("Value")
plt.ylabel("Density")

# Show plots
plt.tight_layout()
plt.show()
