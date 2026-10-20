p = publish('matlab_hw1.m', struct('format','pdf','outputDir','../output'));
movefile(p, fullfile(pwd, 'output', 'homework2.pdf'))