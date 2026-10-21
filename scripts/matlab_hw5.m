%%  Homework 5
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

transition_matrices = jsondecode(fileread(paths(3)));
temperature = 1:0.1:10;

% lambda2 = arrayfun(@())

%%  Question 2: Monte Carlo simulation of the 1D Ising model.
%%  Question 3: Transfer-matrix method for the 1D Ising model.
end

function l = second_eigenvalue(P)
[~,D] = eig(matrix);
[ds,~] = sort(abs(diag(D)),"descend");
l = ds(2);
end