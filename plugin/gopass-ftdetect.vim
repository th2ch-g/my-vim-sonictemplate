" Detect gopass-edit temp files as filetype 'gopass'.
"
" This file lives under plugin/ rather than ftdetect/ because some plugin
" managers (e.g. vim-jetpack) load opt/ plugins via a User-event hook that
" runs after the CLI-argument file's BufRead has already fired during
" vimrc. The autocmd below covers subsequent buffers; the explicit check
" at the bottom catches the very first buffer that Vim opened before this
" script was sourced.
augroup gopass_secret
  autocmd!
  autocmd BufNewFile,BufRead */gopass-edit*/secret*  setfiletype gopass
augroup END

if &filetype ==# '' && expand('%:p') =~# '/gopass-edit[^/]*/secret[^/]*$'
  setfiletype gopass
endif
