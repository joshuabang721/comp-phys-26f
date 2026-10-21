%%  Homework 5
function matlab_hw5(pathstr)
paths = string(jsondecode(fileread(pathstr)));
%%  Question 1: Three-spin Ising model and the transition matrix.

transition_graph_data = jsondecode(fileread(paths(1)));

transition_graph = digraph(transition_graph_data(1,:) + 1, transition_graph_data(2,:) + 1, transition_graph_data(3,:));

figure(1);
hold on;
plot(transition_graph);
title("Transition Graph")
hold off;

%%  Question 2: Monte Carlo simulation of the 1D Ising model.
%%  Question 3: Transfer-matrix method for the 1D Ising model.
end

function a = a(pathstr)
a = 5.0
end