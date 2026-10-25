%%  Homework 5

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw5(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Question 1: Three-spin Ising model and the transition matrix.

%   Transition graph

transition_graph_data = jsondecode(fileread(paths(1)));

transition_graph = digraph(transition_graph_data(1,:) + 1, transition_graph_data(2,:) + 1, transition_graph_data(3,:));

figure(1);
hold on;
plot(transition_graph);
title("Transition Graph")
hold off;

%   Transition matrix and stationary distribution

beta = [10.0,1.0,0.1];
transition_matrices = jsondecode(fileread(paths(2)));

for i = 1:3
    matrix = squeeze(transition_matrices(i,:,:));

    [V,D] = eig(matrix);
    [~,ind] = sort(abs(diag(D)),"descend");
    Vs = V(:,ind);
    v_pi = Vs(:,1);
    v_pi = v_pi / sum(v_pi);

    figure(1 + i);
    hold on;
    bar(v_pi);
    xticklabels(["","- - -","- - + ","- + -","- + +","+ - -","+ - +","+ + -","+ + +"]);
    title("Stationary Probability Distribution (beta = " + string(beta(i)) + ")");
    xlabel("Configuration");
    ylabel("Probability");
    hold off;
end

disp("The stationary distributions are concentrated at the two fully aligned configurations and evenly distributed across the remaining configurations. As the temperature decreases, the concentration near the aligned configurations grows stronger. At very low temperatures, the concentration is so strong that the probability is only concentrated at one of the two aligned configurations.");

%   Relaxation time

temperature = jsondecode(fileread(paths(3)));
transition_matrices = jsondecode(fileread(paths(4)));

lambda2 = arrayfun(@(i) second_eigenvalue(squeeze(transition_matrices(i,:,:))), 1:(length(transition_matrices)));
trelax = -1 ./ log(lambda2');

figure(5);
hold on;
plot(temperature,trelax,'b-');
title("Relaxation Time");
xlabel("Temperature");
ylabel("Time");
hold off;

%%  Question 2: Monte Carlo simulation of the 1D Ising model.

kb = 1;
J = 1;
N = 50;

t = jsondecode(fileread(paths(5)));
energy_density = jsondecode(fileread(paths(6)));
heat_capacity = jsondecode(fileread(paths(7)));
magnet_density = jsondecode(fileread(paths(8)));
magnet_sus = jsondecode(fileread(paths(9)));

b = @(t) 1 ./ (kb*t);
lp = @(t) exp(b(t)*J) + exp(-b(t)*J);
lm = @(t) exp(b(t)*J) - exp(-b(t)*J);
Z = @(t) (lp(t)).^(N) + (lm(t)).^(N);

energy_density_true = @(t) -J .* ((lp(t).^(N-1)).*lm(t) + (lm(t).^(N-1)).*lp(t)) ./ Z(t);
heat_capacity_true = @(t) (4.*kb.*(b(t).*J).^2).*(lp(t).^(2*N-2) - lm(t).^(2*N-2) + 4.*(N-1).*((lp(t).*lm(t)).^(N-2))) ./ ((Z(t)).^2);
magnet_sus_true = @(t) b(t).*((lp(t).^(N-1)).*(exp(b(t).*J) + exp(3.*b(t).*J)) + (lm(t).^(N-1)).*(exp(b(t).*J) - exp(3.*b(t).*J))) ./ Z(t);

e = energy_density_true(t);
c = heat_capacity_true(t);
x = magnet_sus_true(t);

e_mask = abs(e) <= max(abs(energy_density));
c_mask = abs(c) <= max(abs(heat_capacity));
x_mask = abs(x) <= max(abs(magnet_sus));

figure(6);
hold on;
plot(t(e_mask),e(e_mask),'r-',LineWidth=1);
plot(t,energy_density,'bo',MarkerSize=4);
title("Energy Density");
xlabel("Temperature");
ylabel("Density");
legend("Analytical", "Simulated");
hold off;

figure(7);
hold on;
plot(t(c_mask),c(c_mask),'r-',LineWidth=1);
plot(t,heat_capacity,'bo',MarkerSize=4);
ylim([-0.1,inf]);
title("Heat Capacity per Site");
xlabel("Temperature");
ylabel("Capacity");
legend("Analytical", "Simulated");
hold off;

figure(8);
hold on;
yline(0,'r-',LineWidth=1)
plot(t,magnet_density,'bo',MarkerSize=4);
title("Magnetization Density");
xlabel("Temperature");
ylabel("Density");
legend("Analytical", "Simulated");
hold off;

figure(9);
hold on;
plot(t(x_mask),x(x_mask),'r-',LineWidth=1);
plot(t,magnet_sus,'bo',MarkerSize=4);
title("Magnetization Susceptability per Site");
xlabel("Temperature");
ylabel("Susceptability");
legend("Analytical", "Simulated");
hold off;

%%  Question 3: Transfer-matrix method for the 1D Ising model.
end

function l2 = second_eigenvalue(P)
[~,D] = eig(P);
[ds,~] = sort(abs(diag(D)),"descend");
l2 = ds(2);
end