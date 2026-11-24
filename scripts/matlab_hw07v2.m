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
plot(t, x0true, 'r-');
plot(t, x0, 'b-');
title("Trajectory of Node x0(t) (dt = 0.05)");
xlabel("Time");
ylabel("Position");
legend("Analytical", "Simulated", Location="east");
hold off;

ek = jsondecode(fileread(paths(4)));
ep = jsondecode(fileread(paths(5)));
et = jsondecode(fileread(paths(6)));

disp("The system starts away from the equilibrium position x = 0, which means that the system has potential energy at t = 0. As time progresses, the system's potential energy is converted into kinetic energy as it speeds up and approaches equilibrium, and kinetic energy is converted back into potential energy as the system slows down and moves away from equilibrium. These exchanges happen in an oscillating fashion, such that the total energy (Ek + Ep) is conserved; for this system, Et stays at a constant value of approximately 0.307.");

figure(2);
hold on;
plot(t, ek, 'b-');
plot(t, ep, 'r-');
plot(t, et, 'k-');
title("Energy (dt = 0.05)");
xlabel("Time");
ylabel("Energy");
legend("Kinetic", "Potential", "Total");
hold off;

dt = log(jsondecode(fileread(paths(7))));
eps = log(jsondecode(fileread(paths(8))));

figure(3);
hold on;
fplot(@(t) 2*t - 4.6, [-4 -1], 'r-');
plot(dt, eps, 'b-');
title("Error Scaling (log-log)");
xlabel("Time Step dt");
ylabel("Error");
legend("Analytical", "Simulated");
hold off;

%%  Problem 2: Propagation of a localized excitation in the linear chain

time = jsondecode(fileread(paths(9)));
eps00 = jsondecode(fileread(paths(10)));
eps10 = jsondecode(fileread(paths(11)));
eps20 = jsondecode(fileread(paths(12)));
eps30 = jsondecode(fileread(paths(13)));
eps40 = jsondecode(fileread(paths(14)));
eps50 = jsondecode(fileread(paths(15)));

figure(4);
hold on;
plot(eps00);
plot(eps10);
plot(eps20);
plot(eps30);
plot(eps40);
plot(eps50);
title("Local Energy");
xlabel("Space");
ylabel("Energy");
legend("t = 0", "t = 10", "t = 20", "t = 30", "t = 40", "t = 50");
hold off;

disp("Based on the graph, the wavefront seems to travel approximately 10 units of space every 10 units of time, which would mean they have a speed of 1. This matches the calculated value of c = sqrt(k/m) = sqrt(1.0/1.0) = 1.0.");

et = jsondecode(fileread(paths(16)));
eps = jsondecode(fileread(paths(17)));

figure(5);
hold on;
plot(time,et);
plot(time,eps);
title("Total Energy");
xlabel("Time");
ylabel("Energy");
ylim([2 4.5]);
legend("Analytic", "Simulated");
hold off;

disp("Based on the graph, the total energy of the system across time seems to be conserved, both in terms of summing local energy across space and in terms of kinetic plus potential energy.");

%%  Problem 3: Nonlinear FPUT-α chain and recurrence



end