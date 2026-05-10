" Detect gopass-edit temp files as filetype 'gopass'.
" Lives under plugin/ instead of ftdetect/ so it is sourced via Vim's
" runtime! plugin/**/*.vim startup step, which works regardless of whether
" the plugin manager forwards ftdetect to filetype-detect.
augroup gopass_secret
  autocmd!
  autocmd BufNewFile,BufRead */gopass-edit*/secret*  setfiletype gopass
augroup END
