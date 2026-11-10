%%  Homework 10

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw10(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Problem 1: Crank-Nicolson method for the diffusion equation.

x = jsondecode(fileread(paths(1)));
t = jsondecode(fileread(paths(2)));
u_x = jsondecode(fileread(paths(3)));
u_0 = jsondecode(fileread(paths(4)));

figure(1);
hold on;
for i = 1:5
    plot(x,squeeze(u_x(i,:)));
end
title("Diffusion Profile over Time");
xlabel("Position");
ylabel("Diffusion");
legend("t = 0","t = 15","t = 30","t = 45","t = 60");
hold off;

w = 10;
D = 0.5;
figure(2);
hold on;
plot(t,1 ./ sqrt(1 + 4.*D.*t./(w*w)),LineWidth=2);
plot(t,u_0,LineWidth=1);
title("Peak Diffusion  over Time");
xlabel("Time");
ylabel("Diffusion");
legend("True","Simulated");
hold off;

end