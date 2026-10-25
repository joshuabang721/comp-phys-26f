%%  Homework 1

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw1(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  1)  Choose a reference pseudo-random number generator.

disp("The default algorithm used by Rust's standard rand crate is the ChaCha block cipher with 12 rounds. The RNG can be reseeded using the reseed() method.");

%%  2)  Implement a linear congruential generator and basic tests.

prng_samples = jsondecode(fileread(paths(1)));
lcg_samples = jsondecode(fileread(paths(2)));

figure(1);
hold on;
plot((1:length(prng_samples)),prng_samples,'ro');
title("PRNG Sequence");
xlabel("Index k");
ylabel("Value x_k");
hold off;

figure(2);
hold on;
plot((1:length(lcg_samples)),lcg_samples,'bo');
title("LCG Sequence");
xlabel("Index k");
ylabel("Value x_k");
hold off;

figure(3);
hold on;
histogram(prng_samples);
histogram(lcg_samples);
title("RNG Frequency");
xlabel("Value");
ylabel("Frequency");
legend("PRNG (ChaCha)","LCG",Location="southeast");
hold off;

%%  3)  Sampling points in a three-dimensional space.

prng_vector_samples = jsondecode(fileread(paths(3)));
lcg_vector_samples = jsondecode(fileread(paths(4)));

figure(4);
hold on;
plot3(prng_vector_samples(1,:),prng_vector_samples(2,:),prng_vector_samples(3,:),'ro',MarkerSize=2);
title("PRNG 3D Samples");
view(45,30);
hold off;

figure();
hold on;
plot3(lcg_vector_samples(1,:),lcg_vector_samples(2,:),lcg_vector_samples(3,:),'bo',MarkerSize=2);
title("LCG 3D Samples");
view(45,30);
hold off;

%%  4)  Autocorrelation function.

prng_autocorrelation = jsondecode(fileread(paths(5)));
lcg_autocorrelation = jsondecode(fileread(paths(6)));

figure(6);
hold on;
plot((1:length(prng_autocorrelation)),prng_autocorrelation,'r-');
plot((1:length(lcg_autocorrelation)),lcg_autocorrelation,'b-');
title("RNG Autocorrelation");
xlabel("Lag n");
ylabel("Autocorrelation A_n");
legend("PRNG (ChaCha)","LCG",Location="northeast");
hold off;

end