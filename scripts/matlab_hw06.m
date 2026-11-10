%%  Homework 06

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw06(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Problem 1: Initial-error versus memory-load map.

params = jsondecode(fileread(paths(1)));
alpha = jsondecode(fileread(paths(2)));
freq = jsondecode(fileread(paths(3)));
data = jsondecode(fileread(paths(4)));

figure(1);
hold on;
imagesc(alpha,freq,data');
title("Retrieval Overlap (N = " + params(1) + ", R = " + params(2) + ", t_b = " + params(3) + ", t_f = " + params(4) + ")");
xlabel("Memory Load (α)");
ylabel("Initial Error (f)");
colorbar;
hold off;

%%  Problem 2: Temperature versus memory-load map.

params = jsondecode(fileread(paths(5)));
alpha = jsondecode(fileread(paths(6)));
temp = jsondecode(fileread(paths(7)));
data = jsondecode(fileread(paths(8)));

figure(2);
hold on;
imagesc(alpha,temp,data');
title("Retrieval Overlap (N = " + params(1) + ", R = " + params(2) + ", t_b = " + params(3) + ", t_f = " + params(4) + ")");
xlabel("Memory Load (α)");
ylabel("Temperature (T)");
colorbar;
hold off;

end