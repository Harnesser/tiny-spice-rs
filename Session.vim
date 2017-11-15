let SessionLoad = 1
if &cp | set nocp | endif
let s:cpo_save=&cpo
set cpo&vim
map! <S-Insert> <MiddleMouse>
vmap [% [%m'gv``
vmap ]% ]%m'gv``
vmap a% [%v]%
nmap gx <Plug>NetrwBrowseX
nnoremap <silent> <Plug>NetrwBrowseX :call netrw#NetrwBrowseX(expand("<cWORD>"),0)
map <S-Insert> <MiddleMouse>
let &cpo=s:cpo_save
unlet s:cpo_save
set background=dark
set backspace=indent,eol,start
set fileencodings=ucs-bom,utf-8,default,latin1
set helplang=en
set history=50
set matchpairs=(:),{:},[:],<:>
set nomodeline
set mouse=a
set printoptions=paper:a4
set ruler
set runtimepath=~/.vim,~/.vim/bundle/rust.vim,/var/lib/vim/addons,/usr/share/vim/vimfiles,/usr/share/vim/vim74,/usr/share/vim/vimfiles/after,/var/lib/vim/addons/after,~/.vim/bundle/rust.vim/after,~/.vim/after
set suffixes=.bak,~,.swp,.o,.info,.aux,.log,.dvi,.bbl,.blg,.brf,.cb,.ind,.idx,.ilg,.inx,.out,.toc
set termencoding=utf-8
set window=56
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/Projects/tiny-spice-rs
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +112 src/spice.rs
badd +8 ngspice/test_reader.spi
badd +35 bin/tiny-spice.rs
badd +67 src/engine.rs
badd +38 src/analysis.rs
badd +29 tests/test_ird.rs
badd +17 src/circuit.rs
badd +64 tests/test_trans_ir_bridge_loaded.rs
args src/spice.rs ngspice/test_reader.spi
edit bin/tiny-spice.rs
set splitbelow splitright
wincmd _ | wincmd |
split
wincmd _ | wincmd |
split
2wincmd k
wincmd w
wincmd w
set nosplitbelow
set nosplitright
wincmd t
set winheight=1 winwidth=1
exe '1resize ' . ((&lines * 13 + 28) / 57)
exe '2resize ' . ((&lines * 26 + 28) / 57)
exe '3resize ' . ((&lines * 14 + 28) / 57)
argglobal
edit bin/tiny-spice.rs
nnoremap <buffer> <D-R> :RustRun! =join(b:rust_last_rustc_args)erust#AppendCmdLine(' -- ' . join(b:rust_last_args))
nnoremap <buffer> <silent> <D-r> :RustRun
onoremap <buffer> <silent> [[ :call rust#Jump('o', 'Back')
xnoremap <buffer> <silent> [[ :call rust#Jump('v', 'Back')
nnoremap <buffer> <silent> [[ :call rust#Jump('n', 'Back')
onoremap <buffer> <silent> ]] :call rust#Jump('o', 'Forward')
xnoremap <buffer> <silent> ]] :call rust#Jump('v', 'Forward')
nnoremap <buffer> <silent> ]] :call rust#Jump('n', 'Forward')
let s:cpo_save=&cpo
set cpo&vim
noremap <buffer> <F9> :make build
let &cpo=s:cpo_save
unlet s:cpo_save
setlocal keymap=
setlocal noarabic
setlocal autoindent
setlocal balloonexpr=
setlocal nobinary
setlocal bufhidden=
setlocal buflisted
setlocal buftype=
setlocal cindent
setlocal cinkeys=0{,0},!^F,o,O,0[,0]
setlocal cinoptions=L0,(0,Ws,J1,j1
setlocal cinwords=for,if,else,while,loop,impl,mod,unsafe,trait,struct,enum,fn,extern
setlocal colorcolumn=
setlocal comments=s0:/*!,m:\ ,ex:*/,s1:/*,mb:*,ex:*/,:///,://!,://
setlocal commentstring=//%s
setlocal complete=.,w,b,u,t,i
setlocal concealcursor=
setlocal conceallevel=0
setlocal completefunc=
setlocal nocopyindent
setlocal cryptmethod=
setlocal nocursorbind
setlocal nocursorcolumn
setlocal nocursorline
setlocal define=
setlocal dictionary=
setlocal nodiff
setlocal equalprg=
setlocal errorformat=%f:%l:%c:\ %t%*[^:]:\ %m,%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,%-G%f:%l\ %s,%-G%*[\ ]^,%-G%*[\ ]^%*[~],%-G%*[\ ]...,%-G,%-Gerror:\ aborting\ %.%#,%-Gerror:\ Could\ not\ compile\ %.%#,%Eerror:\ %m,%Eerror[E%n]:\ %m,%Wwarning:\ %m,%Inote:\ %m,%C\ %#-->\ %f:%l:%c,%-G%\\s%#Downloading%.%#,%-G%\\s%#Compiling%.%#,%-G%\\s%#Finished%.%#,%-G%\\s%#error:\ Could\ not\ compile\ %.%#,%-G%\\s%#To\ learn\ more\\,%.%#
setlocal expandtab
if &filetype != 'rust'
setlocal filetype=rust
endif
setlocal foldcolumn=0
setlocal foldenable
setlocal foldexpr=0
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldmarker={{{,}}}
setlocal foldmethod=manual
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldtext=foldtext()
setlocal formatexpr=
setlocal formatoptions=croqnlj
setlocal formatlistpat=^\\s*\\d\\+[\\]:.)}\\t\ ]\\s*
setlocal grepprg=
setlocal iminsert=0
setlocal imsearch=2
setlocal include=
setlocal includeexpr=substitute(v:fname,'::','/','g')
setlocal indentexpr=GetRustIndent(v:lnum)
setlocal indentkeys=0{,0},!^F,o,O,0[,0]
setlocal noinfercase
setlocal iskeyword=@,48-57,_,192-255
setlocal keywordprg=
setlocal nolinebreak
setlocal nolisp
setlocal nolist
setlocal makeprg=cargo\ $*
setlocal matchpairs=(:),{:},[:],<:>
setlocal nomodeline
setlocal modifiable
setlocal nrformats=octal,hex
setlocal nonumber
setlocal numberwidth=4
setlocal omnifunc=
setlocal path=
setlocal nopreserveindent
setlocal nopreviewwindow
setlocal quoteescape=\\
setlocal noreadonly
setlocal norelativenumber
setlocal norightleft
setlocal rightleftcmd=search
setlocal noscrollbind
setlocal shiftwidth=4
setlocal noshortname
setlocal smartindent
setlocal softtabstop=4
setlocal nospell
setlocal spellcapcheck=[.?!]\\_[\\])'\"\	\ ]\\+
setlocal spellfile=
setlocal spelllang=en
setlocal statusline=
setlocal suffixesadd=.rs
setlocal swapfile
setlocal synmaxcol=3000
if &syntax != 'rust'
setlocal syntax=rust
endif
setlocal tabstop=4
setlocal tags=
setlocal textwidth=99
setlocal thesaurus=
setlocal noundofile
setlocal nowinfixheight
setlocal nowinfixwidth
setlocal wrap
setlocal wrapmargin=0
silent! normal! zE
let s:l = 24 - ((4 * winheight(0) + 6) / 13)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
24
normal! 05|
wincmd w
argglobal
edit src/spice.rs
nnoremap <buffer> <D-R> :RustRun! =join(b:rust_last_rustc_args)erust#AppendCmdLine(' -- ' . join(b:rust_last_args))
nnoremap <buffer> <silent> <D-r> :RustRun
onoremap <buffer> <silent> [[ :call rust#Jump('o', 'Back')
xnoremap <buffer> <silent> [[ :call rust#Jump('v', 'Back')
nnoremap <buffer> <silent> [[ :call rust#Jump('n', 'Back')
onoremap <buffer> <silent> ]] :call rust#Jump('o', 'Forward')
xnoremap <buffer> <silent> ]] :call rust#Jump('v', 'Forward')
nnoremap <buffer> <silent> ]] :call rust#Jump('n', 'Forward')
let s:cpo_save=&cpo
set cpo&vim
noremap <buffer> <F9> :make build
let &cpo=s:cpo_save
unlet s:cpo_save
setlocal keymap=
setlocal noarabic
setlocal autoindent
setlocal balloonexpr=
setlocal nobinary
setlocal bufhidden=
setlocal buflisted
setlocal buftype=
setlocal cindent
setlocal cinkeys=0{,0},!^F,o,O,0[,0]
setlocal cinoptions=L0,(0,Ws,J1,j1
setlocal cinwords=for,if,else,while,loop,impl,mod,unsafe,trait,struct,enum,fn,extern
setlocal colorcolumn=
setlocal comments=s0:/*!,m:\ ,ex:*/,s1:/*,mb:*,ex:*/,:///,://!,://
setlocal commentstring=//%s
setlocal complete=.,w,b,u,t,i
setlocal concealcursor=
setlocal conceallevel=0
setlocal completefunc=
setlocal nocopyindent
setlocal cryptmethod=
setlocal nocursorbind
setlocal nocursorcolumn
setlocal nocursorline
setlocal define=
setlocal dictionary=
setlocal nodiff
setlocal equalprg=
setlocal errorformat=%f:%l:%c:\ %t%*[^:]:\ %m,%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,%-G%f:%l\ %s,%-G%*[\ ]^,%-G%*[\ ]^%*[~],%-G%*[\ ]...,%-G,%-Gerror:\ aborting\ %.%#,%-Gerror:\ Could\ not\ compile\ %.%#,%Eerror:\ %m,%Eerror[E%n]:\ %m,%Wwarning:\ %m,%Inote:\ %m,%C\ %#-->\ %f:%l:%c,%-G%\\s%#Downloading%.%#,%-G%\\s%#Compiling%.%#,%-G%\\s%#Finished%.%#,%-G%\\s%#error:\ Could\ not\ compile\ %.%#,%-G%\\s%#To\ learn\ more\\,%.%#
setlocal expandtab
if &filetype != 'rust'
setlocal filetype=rust
endif
setlocal foldcolumn=0
setlocal foldenable
setlocal foldexpr=0
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldmarker={{{,}}}
setlocal foldmethod=manual
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldtext=foldtext()
setlocal formatexpr=
setlocal formatoptions=croqnlj
setlocal formatlistpat=^\\s*\\d\\+[\\]:.)}\\t\ ]\\s*
setlocal grepprg=
setlocal iminsert=0
setlocal imsearch=2
setlocal include=
setlocal includeexpr=substitute(v:fname,'::','/','g')
setlocal indentexpr=GetRustIndent(v:lnum)
setlocal indentkeys=0{,0},!^F,o,O,0[,0]
setlocal noinfercase
setlocal iskeyword=@,48-57,_,192-255
setlocal keywordprg=
setlocal nolinebreak
setlocal nolisp
setlocal nolist
setlocal makeprg=cargo\ $*
setlocal matchpairs=(:),{:},[:],<:>
setlocal nomodeline
setlocal modifiable
setlocal nrformats=octal,hex
setlocal nonumber
setlocal numberwidth=4
setlocal omnifunc=
setlocal path=
setlocal nopreserveindent
setlocal nopreviewwindow
setlocal quoteescape=\\
setlocal noreadonly
setlocal norelativenumber
setlocal norightleft
setlocal rightleftcmd=search
setlocal noscrollbind
setlocal shiftwidth=4
setlocal noshortname
setlocal smartindent
setlocal softtabstop=4
setlocal nospell
setlocal spellcapcheck=[.?!]\\_[\\])'\"\	\ ]\\+
setlocal spellfile=
setlocal spelllang=en
setlocal statusline=
setlocal suffixesadd=.rs
setlocal swapfile
setlocal synmaxcol=3000
if &syntax != 'rust'
setlocal syntax=rust
endif
setlocal tabstop=4
setlocal tags=
setlocal textwidth=99
setlocal thesaurus=
setlocal noundofile
setlocal nowinfixheight
setlocal nowinfixwidth
setlocal wrap
setlocal wrapmargin=0
silent! normal! zE
let s:l = 1 - ((0 * winheight(0) + 13) / 26)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
1
normal! 0
wincmd w
argglobal
edit src/engine.rs
nnoremap <buffer> <D-R> :RustRun! =join(b:rust_last_rustc_args)erust#AppendCmdLine(' -- ' . join(b:rust_last_args))
nnoremap <buffer> <silent> <D-r> :RustRun
onoremap <buffer> <silent> [[ :call rust#Jump('o', 'Back')
xnoremap <buffer> <silent> [[ :call rust#Jump('v', 'Back')
nnoremap <buffer> <silent> [[ :call rust#Jump('n', 'Back')
onoremap <buffer> <silent> ]] :call rust#Jump('o', 'Forward')
xnoremap <buffer> <silent> ]] :call rust#Jump('v', 'Forward')
nnoremap <buffer> <silent> ]] :call rust#Jump('n', 'Forward')
let s:cpo_save=&cpo
set cpo&vim
noremap <buffer> <F9> :make build
let &cpo=s:cpo_save
unlet s:cpo_save
setlocal keymap=
setlocal noarabic
setlocal autoindent
setlocal balloonexpr=
setlocal nobinary
setlocal bufhidden=
setlocal buflisted
setlocal buftype=
setlocal cindent
setlocal cinkeys=0{,0},!^F,o,O,0[,0]
setlocal cinoptions=L0,(0,Ws,J1,j1
setlocal cinwords=for,if,else,while,loop,impl,mod,unsafe,trait,struct,enum,fn,extern
setlocal colorcolumn=
setlocal comments=s0:/*!,m:\ ,ex:*/,s1:/*,mb:*,ex:*/,:///,://!,://
setlocal commentstring=//%s
setlocal complete=.,w,b,u,t,i
setlocal concealcursor=
setlocal conceallevel=0
setlocal completefunc=
setlocal nocopyindent
setlocal cryptmethod=
setlocal nocursorbind
setlocal nocursorcolumn
setlocal nocursorline
setlocal define=
setlocal dictionary=
setlocal nodiff
setlocal equalprg=
setlocal errorformat=%f:%l:%c:\ %t%*[^:]:\ %m,%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,%-G%f:%l\ %s,%-G%*[\ ]^,%-G%*[\ ]^%*[~],%-G%*[\ ]...,%-G,%-Gerror:\ aborting\ %.%#,%-Gerror:\ Could\ not\ compile\ %.%#,%Eerror:\ %m,%Eerror[E%n]:\ %m,%Wwarning:\ %m,%Inote:\ %m,%C\ %#-->\ %f:%l:%c,%-G%\\s%#Downloading%.%#,%-G%\\s%#Compiling%.%#,%-G%\\s%#Finished%.%#,%-G%\\s%#error:\ Could\ not\ compile\ %.%#,%-G%\\s%#To\ learn\ more\\,%.%#
setlocal expandtab
if &filetype != 'rust'
setlocal filetype=rust
endif
setlocal foldcolumn=0
setlocal foldenable
setlocal foldexpr=0
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldmarker={{{,}}}
setlocal foldmethod=manual
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldtext=foldtext()
setlocal formatexpr=
setlocal formatoptions=croqnlj
setlocal formatlistpat=^\\s*\\d\\+[\\]:.)}\\t\ ]\\s*
setlocal grepprg=
setlocal iminsert=0
setlocal imsearch=0
setlocal include=
setlocal includeexpr=substitute(v:fname,'::','/','g')
setlocal indentexpr=GetRustIndent(v:lnum)
setlocal indentkeys=0{,0},!^F,o,O,0[,0]
setlocal noinfercase
setlocal iskeyword=@,48-57,_,192-255
setlocal keywordprg=
setlocal nolinebreak
setlocal nolisp
setlocal nolist
setlocal makeprg=cargo\ $*
setlocal matchpairs=(:),{:},[:],<:>
setlocal nomodeline
setlocal modifiable
setlocal nrformats=octal,hex
setlocal nonumber
setlocal numberwidth=4
setlocal omnifunc=
setlocal path=
setlocal nopreserveindent
setlocal nopreviewwindow
setlocal quoteescape=\\
setlocal noreadonly
setlocal norelativenumber
setlocal norightleft
setlocal rightleftcmd=search
setlocal noscrollbind
setlocal shiftwidth=4
setlocal noshortname
setlocal smartindent
setlocal softtabstop=4
setlocal nospell
setlocal spellcapcheck=[.?!]\\_[\\])'\"\	\ ]\\+
setlocal spellfile=
setlocal spelllang=en
setlocal statusline=
setlocal suffixesadd=.rs
setlocal swapfile
setlocal synmaxcol=3000
if &syntax != 'rust'
setlocal syntax=rust
endif
setlocal tabstop=4
setlocal tags=
setlocal textwidth=99
setlocal thesaurus=
setlocal noundofile
setlocal nowinfixheight
setlocal nowinfixwidth
setlocal wrap
setlocal wrapmargin=0
silent! normal! zE
let s:l = 1 - ((0 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
1
normal! 0
wincmd w
3wincmd w
exe '1resize ' . ((&lines * 13 + 28) / 57)
exe '2resize ' . ((&lines * 26 + 28) / 57)
exe '3resize ' . ((&lines * 14 + 28) / 57)
tabnext 1
if exists('s:wipebuf')
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToO
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
