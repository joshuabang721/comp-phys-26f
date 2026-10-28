%%  Homework 4

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw4(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Question 1: Markov-chain Monte Carlo and the Metropolis–Hastings algorithm.

markov_chain = jsondecode(fileread(paths(1)));

figure(1);
hold on;
histogram(markov_chain,Normalization='pdf');
title("Markov Chain Distribution");
xlabel("Position");
ylabel("Probability");
xlim([-2,2]);
hold off;

%%  Question 2: Metropolis sampling: two interacting particles in a box.

heat_map_x1 = jsondecode(fileread(paths(2)));
heat_map_x2 = jsondecode(fileread(paths(3)));

figure(2);
hold on;
histogram2(heat_map_x1,heat_map_x2,Normalization='pdf');
view(-45,30);
title("Two Particles Distribution (λ = 1.0)");
zlabel("Probability");
hold off;

lambda = jsondecode(fileread(paths(4)));
avg_dist = jsondecode(fileread(paths(5)));

figure(3);
hold on;
plot(lambda,avg_dist,'b-');
title("Average Separation");
xlabel("λ");
ylabel("|x1 - x2|");
hold off;

end