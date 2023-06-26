% Weizhi Wang & Arun Kamath, NTNU, Mar,22,2017
clear all
clc
close all

% Excerpts
% ...KOORDSYS 23   % UTM sone 33 basert pa EUREF89/WGS84
% ...ORIGO-NE 0  0 % Note the coordinates are [N,E], i.e.[y,x]
% ...ENHET 0.01    % Note that this is the unit and scale, i.e. values*0.01
% ...MIN-NE 7579658  702593
% ...MAX-NE 7634973  735378

% fid=fopen('test.sos','r');
fid=fopen('Bergen_full.sos','r');
ORIGONE=[0 0];
ENHET=0.01;
mm=0;
nn=0;
DM=1.4; % (m) The distance between the sjokartnull and the Middel h?ybann (HMV) (kystkonturnull)

while ~feof(fid)
    
    strl=fgets(fid);
    if strfind(strl,'DYBDE') > 0
        d_no=str2num(strl(strfind(strl,' ')+1:end));
        tline=fgetl(fid);
        while sum(strfind(tline,'.'))~=1
            while (sum(strfind(tline,'.')>1) && sum(strfind(tline,'.')<=6)) || sum(strfind(tline,':'))>0
                tline=fgetl(fid); % we skip those useless lines, as for checking criteria, check Modification.txt
            end
            
            input=tline;
            N_O=strsplit(input);   % We can't specify which columns to read for coordinates, as the number of digits change,
            % so we separate the lines with space and read the first two parts, which are coordinates
            mm=mm+1;
            x(mm)=str2num(N_O{2}); % Not that the coordinate is [N,E], namely [y,x], we need to switch them
            y(mm)=str2num(N_O{1}); % in some files, it is N_O{2} and N_O{1}
            d(mm)=d_no;
            tline=fgetl(fid);
        end
    else
        if strfind(strl,'Kystkontur') > 0
            d_ky=DM;
            tline=fgetl(fid);
            while sum(strfind(tline,'.'))~=1
                while (sum(strfind(tline,'.')>1) && sum(strfind(tline,'.')<=6)) || sum(strfind(tline,':'))>0
                    tline=fgetl(fid); % we skip those useless lines, as for checking criteria, check Modification.txt
                end
                
                input=tline;
                N_O=strsplit(input);   % We can't specify which columns to read for coordinates, as the number of digits change,
                % so we separate the lines with space and read the first two parts, which are coordinates
                mm=mm+1;
                x(mm)=str2num(N_O{2}); % Not that the coordinate is [N,E], namely [y,x], we need to switch them
                y(mm)=str2num(N_O{1}); % in some files, it is N_O{2} and N_O{1}
                d(mm)=d_ky;
                tline=fgetl(fid);
            end
        end
    end
end

NP=mm;
fclose(fid);
nxyd=zeros(NP,4); %nxyd indicates point number, north, east and depth coordinates
for m=1:NP
    nxyd(m,1)=m;
    nxyd(m,2)=x(m)*ENHET+ORIGONE(2); % Not that the coordinate origin is [N,E], namely [y,x], we need to switch them
    nxyd(m,3)=y(m)*ENHET+ORIGONE(1);
    nxyd(m,4)=d(m);
end

%% xyz
maxd=max(nxyd(:,4));
xyz_0=nxyd(:,2:4);
xyz=[nxyd(:,2) nxyd(:,3) nxyd(:,4)]; % change the depth value to negative and rise till min=0
dlmwrite('geo.dat',xyz,'delimiter',' ','precision','%.6f')

%% Visualization
tri = delaunay(xyz(:,1),xyz(:,2));
h = trisurf(tri, xyz(:,1),xyz(:,2),xyz(:,3));

