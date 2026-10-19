root = matlabroot;

p = fullfile(root, 'sys', 'namespace', 'docbook', ...
    'v4', 'xsl', 'fo', 'docbook_rptgen.xsl');

fprintf("MATLAB root: %s\n", root);
fprintf("Stylesheet exists: %d\n", exist(p, 'file'));
pwd