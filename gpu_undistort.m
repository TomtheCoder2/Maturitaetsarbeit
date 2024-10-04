% read image
I = imread('./code/input_image.png');

% // Camera matrix and distortion coefficients from calibration
  %const CAMERA_MATRIX: [[f64; 3]; 3] = [
  %    [458.11615111, 0.0, 367.97913048],
  %    [0.0, 458.19122834, 272.04155943],
  %    [0.0, 0.0, 1.0],
  %];
  %const DIST_COEFFS: [f64; 5] = [-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724];

camera_matrix = [458.11615111, 0.0, 367.97913048; 0.0, 458.19122834, 272.04155943; 0.0, 0.0, 1.0];
dist_coeffs = [-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724];
% // Camera matrix values
  %    let fx = camera_matrix[0][0] as f64;
  %    let fy = camera_matrix[1][1] as f64;
  %    let cx = camera_matrix[0][2] as f64;
  %    let cy = camera_matrix[1][2] as f64;
  %
  %    // Distortion coefficients
  %    let k1 = dist_coeffs[0];
  %    let k2 = dist_coeffs[1];
  %    let p1 = dist_coeffs[2];
  %    let p2 = dist_coeffs[3];
  %    let k3 = dist_coeffs[4];

fx = camera_matrix(1, 1);
fy = camera_matrix(2, 2);
cx = camera_matrix(1, 3);
cy = camera_matrix(2, 3);

k1 = dist_coeffs(1);
k2 = dist_coeffs(2);
p1 = dist_coeffs(3);
p2 = dist_coeffs(4);
k3 = dist_coeffs(5);


% \[ x_{\text{radial}} = x_{\text{normalized}} \cdot (1 + k_1 \cdot r^2 + k_2 \cdot r^4 + k_3 \cdot r^6) \]
  %\[ y_{\text{radial}} = y_{\text{normalized}} \cdot (1 + k_1 \cdot r^2 + k_2 \cdot r^4 + k_3 \cdot r^6) \]
  %
  %\\- \( x_{\text{normalized}} \) and \( y_{\text{normalized}} \) are the normalized coordinates.
  %\\- \( r^2 = x_{\text{normalized}}^2 + y_{\text{normalized}}^2 \) is the squared radial distance.
  %\\- \( k_1, k_2, k_3 \) are the radial distortion coefficients.
  %
  %\[ x_{\text{tangential}} = 2 \cdot p_1 \cdot x_{\text{normalized}} \cdot y_{\text{normalized}} + p_2 \cdot (r^2 + 2 \cdot x_{\text{normalized}}^2) \]
  %\[ y_{\text{tangential}} = p_1 \cdot (r^2 + 2 \cdot y_{\text{normalized}}^2) + 2 \cdot p_2 \cdot x_{\text{normalized}} \cdot y_{\text{normalized}} \]
  %
  %- \( p_1 \) and \( p_2 \) are the tangential distortion coefficients.
  %
  %\[ x_{\text{distorted}} = x_{\text{radial}} + x_{\text{tangential}} \]
  %\[ y_{\text{distorted}} = y_{\text{radial}} + y_{\text{tangential}} \]

% undistort image
function [x_distorted, y_distorted] = undistort_coordinates(x_normalized, y_normalized, k1, k2, k3, p1, p2)

    % Calculate squared radial distance
    r2 = x_normalized.^2 + y_normalized.^2;
    r4 = r2.^2;
    r6 = r4 .* r2;

    % Apply radial distortion
    radial_distortion = 1 + k1 * r2 + k2 * r4 + k3 * r6;
    x_radial = x_normalized .* radial_distortion;
    y_radial = y_normalized .* radial_distortion;

    % Apply tangential distortion
    x_tangential = 2 * p1 * x_normalized .* y_normalized + p2 * (r2 + 2 * x_normalized.^2);
    y_tangential = p1 * (r2 + 2 * y_normalized.^2) + 2 * p2 * x_normalized .* y_normalized;

    % Distorted coordinates
    x_distorted = x_radial + x_tangential;
    y_distorted = y_radial + y_tangential;
end

% Normalize pixel coordinates
x_normalized = (1:size(I, 2)) / fx - cx / fx;
y_normalized = (1:size(I, 1)) / fy - cy / fy;

% Undistort pixel coordinates
[x_distorted, y_distorted] = undistort_coordinates(x_normalized, y_normalized, k1, k2, k3, p1, p2);

% Map distorted coordinates to original image
I_undistorted = zeros(size(I), 'uint8');
for y = 1:size(I, 1)
    for x = 1:size(I, 2)
        x_dist = round(x_distorted(x) * fx + cx);
        y_dist = round(y_distorted(y) * fy + cy);

        if x_dist >= 1 && x_dist <= size(I, 2) && y_dist >= 1 && y_dist <= size(I, 1)
            I_undistorted(y, x, :) = I(y_dist, x_dist, :);
        end
    end
end
