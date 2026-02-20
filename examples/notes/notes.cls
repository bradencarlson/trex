% Notes class file. 
% Written by Braden Carlson 2024. 
% Used primarily to keep track of two different versions of 
% a given document (usually my personal lecture notes), one in 'presentation'
% format and another in 'nopresentation' format. 
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{notes}


\newif\ifpresentation

\DeclareOption{presentation}{
	\presentationtrue
}
\DeclareOption{nopresentation}{
	\presentationfalse
}

\ExecuteOptions{presentation}
\ProcessOptions\relax

\LoadClass{article}
\usepackage{mathcommands}


\ifpresentation
    \pagestyle{empty}
    \setlength{\paperwidth}{11in}
    \setlength{\paperheight}{8.5in}
    \setlength{\voffset}{-36pt}
    \setlength{\hoffset}{0pt}
    \setlength{\oddsidemargin}{-.5in}
    \setlength{\topmargin}{-30pt}
    \setlength{\headheight}{12pt}
    \setlength{\headsep}{18pt}
    \setlength{\textheight}{542pt}

%     \setlength{\textwidth}{542pt}
%     \setlength{\marginparwidth}{2.25in}
%     \setlength{\marginparsep}{.25in}

    \setlength{\textwidth}{722pt}
%     \setlength{\marginparwidth}{2.25in}
%     \setlength{\marginparsep}{.25in}

\else
    \setlength{\paperwidth}{8.5in}
    \setlength{\paperheight}{11in}
    \setlength{\hoffset}{0pt}
    \setlength{\voffset}{0pt}
    \setlength{\oddsidemargin}{0pt}
    \setlength{\topmargin}{-36pt}
    \setlength{\headheight}{12pt}
    \setlength{\headsep}{24pt}
    \setlength{\textwidth}{6.5in}
    \setlength{\textheight}{9in}
\fi

\long\def\phantom#1#2{\ifpresentation #1 \else #2 \fi}

% define two new lengths to use in the content command. 
% The main length will be the width of the minipage which 
% will be used fo rthe main content. The comment length 
% will be the length of the minipage that will contain the 
% comments. 
\newlength{\mainlength}
\newlength{\commentlength}

% set the default values to 7.25 inches for the main 
% length and 2.5 inches for the comment length. 
\setlength{\mainlength}{7.25in}
\setlength{\commentlength}{2.25in}

% define the content command, which takes 3 arguments, 
% as follows: 
%     - the presentation content
%     - the solution content
%     - the comment content
\long\def\content#1#2#3#4{
    \ifpresentation
        \def\argtwo{#2}
        \def\argfour{#4}
        \begin{minipage}[t]{\mainlength}
        \vspace{0pt}#1\phantom{#2}{#3}
        \end{minipage}\hfill\ifx\argfour\empty\else\fbox{\begin{minipage}[t]{\commentlength}
        \vspace{0pt}#4
        \end{minipage}}\fi
    \else
        #1
        \phantom{#2}{#3}
    \fi
}


\endinput
