initialState =: <(<<'/'),<a:,<0
input =: initialState,~|. }: LF chopstring (1!:1 < 'input.txt')

eq =: =&:$ * *./@:(] = ($~$))

NB. State is a box consisting of two fields: The current working directory, and
NB. the explored filesystem. The current working directory is itself a box
NB. consisting of an array of boxed strings
process =: dyad define
  cwd   =. >{.&.>y
  fs    =. >{:&.>y
  if. '$ cd /' eq >x do.
    cwd =. <<'/'
  elseif. '$ cd ..' eq >x do.
    cwd =. }:&.> cwd
  elseif. '$ cd' eq 4$>x do.
    cwd =. <(>cwd),(5}.&.>x)
  elseif. '$ ls' eq 4$>x do.
    NB. Ignore
  elseif. 'dir' eq 3$>x do.
    NB. Ignore
  else.
    value =: <{. _999 ". >x
    fs =. <(>fs),.|:,/"1({{ 2 1 $(<y),value }}\>cwd)
  end.
  <cwd,fs
)

structure =: (0;1) {:: process/ input
sizes     =: (~.{.structure),:+/@:>&.>({. </. {:) structure
small     =: ((<:&100000)@:>@:{: # |:) sizes
echo +/>{:|: small
