%%  Homework 07v2

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw07v2(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Problem 1: Linear-chain benchmark and error scaling.

t = jsondecode(fileread(paths(1)));
x0 = jsondecode(fileread(paths(2)));
x0true = jsondecode(fileread(paths(3)));

figure(1);
hold on;
plot(t,x0,'b-');
plot(t,x0true,'r-');
title("Trajectory of Node x0(t)");
xlabel("Time");
ylabel("Position");
legend("Simulated", "Analytical");
hold off;

ek = jsondecode(fileread(paths(4)));
ep = jsondecode(fileread(paths(5)));
et = jsondecode(fileread(paths(6)));

figure(2);
hold on;
plot(t,ek,'b-');
plot(t,ep,'r-');
plot(t,et,'k-');
title("Energy");
xlabel("Time");
ylabel("Energy");
legend("Kinetic", "Potential", "Total");
hold off;

dt = jsondecode(fileread(paths(7)));
eps = jsondecode(fileread(paths(8)));

figure(3);
hold on;
plot(dt,eps,'b-');
title("Error Scaling");
xlabel("Time Step dt");
ylabel("Error");
hold off;

end