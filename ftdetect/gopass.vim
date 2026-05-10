augroup gopass_secret
  au!
  au BufNewFile,BufRead */gopass-edit*/secret\ #*  setfiletype gopass
augroup END
