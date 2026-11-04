%%  Homework 10

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw10(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Question 1: Three-spin Ising model and the transition matrix.

t = jsondecode(fileread(paths(1)));
E = jsondecode(fileread(paths(2)));

figure(1);
hold on;
plot(t,E);
title("Recurrence of Nonlinear FPUT Chain");
xlabel("Time");
ylabel("Energy");
hold off;

end