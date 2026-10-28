%%  Homework 7
function matlab_hw7(pathstr)
paths = string(jsondecode(fileread(pathstr)));
%%  Problem 1: Simple harmonic oscillator.

%   Trajectory.

t = 0:0.1:30;
x = jsondecode(fileread(paths(1)));
x_true = cos(t);

figure(1);
hold on;
plot(t,x,'b-');
plot(t,x_true,'r-');
title("Simple Harmonic Oscillator Trajectory");
xlabel("Time");
ylabel("Position");
legend("Velocity Verlet","True Path");
hold off;

%   Energy conservation.

t = 0:0.1:30;
ek = jsondecode(fileread(paths(2)));
ep = jsondecode(fileread(paths(3)));
et = jsondecode(fileread(paths(4)));
e_exact = et(1);

figure(2);
hold on;
plot(t,ek,'b-');
plot(t,ep,'g-');
plot(t,et,'r-');
yline(e_exact,'k-');
title("Simple Harmonic Oscillator Energy");
xlabel("Time");
ylabel("Energy");
legend("Kinetic Energy","Potential Energy","Total Energy","Exact Energy (E_T(0))", Location="southeast");
hold off;

%   Error scaling.

dt = 0.025:0.025:0.2;
error = jsondecode(fileread(paths(5)));

figure(3);
hold on;
plot(log(dt),log(error),'b-');
title("SHO Velocity Verlet Error");
xlabel("log(Size of Time Step)");
ylabel("log(Error)");
hold off;

%%  Problem 2: Periodic Chain of Couple Oscillators.

%   Trajectory.

t = 0:0.1:30;
x1 = jsondecode(fileread(paths(6)));
x2 = jsondecode(fileread(paths(7)));
x3 = jsondecode(fileread(paths(8)));

figure(4);
hold on;
plot(t,x1,'r-');
plot(t,x2,'g-');
plot(t,x3,'b-');
title("Chain Coupled Oscillator Trajectories");
xlabel("Time");
ylabel("Position");
legend("Particle 1","Particle 2","Particle 3");
hold off;

%   Energy conservation.

t = 0:0.1:30;
ek = jsondecode(fileread(paths(9)));
ep = jsondecode(fileread(paths(10)));
et = jsondecode(fileread(paths(11)));
e_exact = et(1);

figure(5);
hold on;
plot(t,ek,'b-');
plot(t,ep,'g-');
plot(t,et,'r-');
yline(e_exact,'k-');
title("Simple Harmonic Oscillator Energy");
xlabel("Time");
ylabel("Energy");
legend("Kinetic Energy","Potential Energy","Total Energy","Exact Energy (E_T(0))", Location="southeast");
hold off;

%   Error scaling.

dt = 0.025:0.025:0.2;
error = jsondecode(fileread(paths(12)));

figure(6);
hold on;
plot(log(dt),log(error),'b-');
title("CCO Velocity Verlet Error");
xlabel("log(Size of Time Step)");
ylabel("log(Error)");
hold off;

end