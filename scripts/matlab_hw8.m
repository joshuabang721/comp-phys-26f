%%  Homework 8

%   GitHub: https://github.com/joshuabang721/comp-phys-26f

function matlab_hw8(pathstr)

paths = string(jsondecode(fileread(pathstr)));

%%  Question 1: Three-spin Ising model and the transition matrix.

dt = [0.06,0.10,0.14];
t = [0,5,10,15,20];
times = jsondecode(fileread(paths(1)));
waves = jsondecode(fileread(paths(2)));

for i = 1:3
    time = squeeze(times(i,:));
    n = time(time = t);
    wave = squeeze(wave(i,:,:));
    x_n = wave(n,:);

    figure(i)
    hold on;
    for j = 1:length(x_n)
        plot(x_n(j,:),'b-');
    end
    title("Lax Wave Propogation");
    xlabel("Position");
    ylabel("Wave");
    legend("t = 0","t = 5","t = 10","t = 15","t = 20");
    hold off;
end

end