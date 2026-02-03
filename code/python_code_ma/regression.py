import matplotlib.pyplot as plt
import numpy as np

# positions = [[206, 290], [260, 274], [179, 302], [141, 319], [102, 339], [91, 359], [363, 238]]
# positions = [[31, 361], [96, 330], [155, 310], [219, 286], [270, 269], [343, 244], [37, 361]]

# positions = [[55, 352], [54, 352], [91, 335], [150, 313], [205, 291], [254, 270], [296, 256]] # good data for doc
# positions = [[0, 357], [50, 340], [100, 317], [150, 303], [200, 282], [250, 269], [300, 246]] # good data for doc (rs485)
# positions = [[1, 361], [47, 348], [96, 330], [144, 315], [211, 289], [254, 270], [303, 257]]
# positions = [[210, 283], [50, 345], [100, 325], [150, 305], [200, 286], [250, 269], [300, 250]]
# positions = [[262, 245], [50, 323], [100, 310], [150, 289], [200, 266], [250, 245], [300, 232]]
# positions = [[261, 268], [58, 349], [95, 334], [158, 312], [212, 289], [249, 277], [298, 257]]
# positions = [[0, 361], [50, 342], [100, 322], [150, 305], [200, 289], [250, 268], [300, 251]]

# motor pos, ir
positions = [[985, 362], [818, 342], [668, 319], [533, 300], [407, 282], [223, 254], [855, 347]]

def fit_regression(x, y, degree=3):
    """Fits linear and polynomial regressions."""
    coeffs_linear = np.polyfit(x, y, 1)
    coeffs_poly = np.polyfit(x, y, degree)
    return coeffs_linear, coeffs_poly


def format_polynomial(coeffs):
    """Formats a polynomial equation string."""
    terms = [f"{coeff:.4f}x^{i}" if i > 0 else f"{coeff:.4f}" for i, coeff in enumerate(reversed(coeffs))]
    return " + ".join(terms[::-1])


def format_polynomial10(coeffs):
    """Formats a polynomial equation string."""
    terms = [f"{coeff:.10f}x^{i}" if i > 0 else f"{coeff:.10f}" for i, coeff in enumerate(reversed(coeffs))]
    # let x = -0.0002964358 * y.powi(3) + 0.2742589884 * y.powi(2) - 86.9730877714 * y + 9594.7333153706;
    terms_rust = [f"{coeff:.10f} * y.powi({i})" if i > 0 else f"{coeff:.10f}" for i, coeff in
                  enumerate(reversed(coeffs))]
    print(f"let x = {' + '.join(terms_rust[::-1])};")
    return " + ".join(terms[::-1])


def plot_regression(ax, x, y, coeffs_linear, coeffs_poly, title):
    """Plots data points and regression curves."""
    x_vals = np.linspace(min(x), max(x), 500)
    y_linear = np.poly1d(coeffs_linear)(x_vals)
    y_poly = np.poly1d(coeffs_poly)(x_vals)

    ax.scatter(x, y, color="red", label="Data Points")
    ax.plot(x_vals, y_linear, label=f"Linear Fit: {coeffs_linear[0]:.4f}x + {coeffs_linear[1]:.4f}")
    ax.plot(x_vals, y_poly, label=f"Polynomial Fit (Degree {len(coeffs_poly) - 1}): {format_polynomial(coeffs_poly)}")
    ax.set_title(title)
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.legend()
    ax.grid()


# Separate x and y for original and swapped
x, y = zip(*positions)
y_swapped, x_swapped = x, y

# Perform regressions
coeffs_linear, coeffs_poly = fit_regression(x, y)
coeffs_linear_swapped, coeffs_poly_swapped = fit_regression(x_swapped, y_swapped)

# Plot side-by-side
fig, axs = plt.subplots(1, 2, figsize=(12, 6))

# Plot original
plot_regression(axs[0], x, y, coeffs_linear, coeffs_poly, "Regression Fits (X vs Y)")

# Plot swapped
plot_regression(axs[1], x_swapped, y_swapped, coeffs_linear_swapped, coeffs_poly_swapped, "Regression Fits (Y vs X)")

# top=0.952,
# bottom=0.078,
# left=0.046,
# right=0.99,
# hspace=0.2,
# wspace=0.103

plt.tight_layout()
plt.subplots_adjust(top=0.952, bottom=0.078, left=0.046, right=0.99, hspace=0.2, wspace=0.103)

# Print equations
print("Original (X vs Y):")
print(f"  Linear Fit Equation: y = {coeffs_linear[0]:.10f}x + {coeffs_linear[1]:.10f}")
print(f"  Polynomial Fit Equation (Degree {len(coeffs_poly) - 1}): {format_polynomial(coeffs_poly)}")

print("\nSwapped (Y vs X):")
print(f"  Linear Fit Equation: x = {coeffs_linear_swapped[0]:.10f}y + {coeffs_linear_swapped[1]:.10f}")
print(f"  Polynomial Fit Equation (Degree {len(coeffs_poly_swapped) - 1}): {format_polynomial10(coeffs_poly_swapped)}")
print(coeffs_poly_swapped)
plt.show()
