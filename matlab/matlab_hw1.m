function matlab_hw1(pathstr)
%%  Homework 1
paths = split(pathstr, ",");
%%  1)  Choose a reference pseudo-random number generator.
disp("The default algorithm used by Rust's standard rand crate is the ChaCha block cipher with 12 rounds. The RNG can be reseeded using the reseed() method.");

text = fileread(paths(1));
random_samples = jsondecode(text);

figure(1);
hold on;
plot((1:length(random_samples)),random_samples);
hold off;
%%  2)  Implement a linear congruential generator and basic tests
%%  3)  Sampling points in a three-dimensional space.
%%  4)  Autocorrelation function.
end