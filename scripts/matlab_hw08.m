%%  Homework 08

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw08(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Question 1: Three-spin Ising model and the transition matrix.

dt = [0.06,0.10,0.14];
t = [0,5,10,15,20];
x = -20:0.1:50;

times = jsondecode(fileread(paths(1)));
waves = jsondecode(fileread(paths(2)));

for i = 1:3
    time = cell2mat(squeeze(times(i,:)));
    n = floor((t) ./ dt(i) + 1)
    time(n)
    wave = cell2mat(squeeze(waves(i,:,:)));
    x_n = wave(n,:);
    x_n(1,:)

    figure(i)
    hold on;
    for j = 1:5
        plot(x,squeeze(x_n(j,:)));
    end
    title("Lax Wave Propagation (dt = " + string(dt(i)) + ")");
    xlabel("Position");
    ylabel("Wave");
    legend("t = 0","t = 5","t = 10","t = 15","t = 20");
    hold off;
end

end