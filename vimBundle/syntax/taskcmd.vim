" Vim syntax file
" Language:   Task definitions for TaskCommander
" Filenames:  *.taskcmd
" URL:        https://github.com/EngosSoftware/task-commander
" License:    Apache-2.0, MIT

if exists('b:current_syntax')
  finish
endif

let s:cpo_save = &cpo
set cpo&vim

syn case match

" Inline code `...`
syn region taskcmdInlineCode
	       \ matchgroup=taskcmdPunctuation
	       \ start=/`/ end=/`/ oneline contained

" Interpolation {{ ... }}
syn match taskcmdInterpolation /{{[^}]*}}/

" Delimited indentation
syn match taskcmdDot /^\s*\./
          \ nextgroup=taskcmdNode,taskcmdVariable,taskcmdDesc,taskcmdNote

" Any node .<name>
syn match taskcmdNode /\S\+/ contained

" Variable node .<NAME> 
syn match taskcmdVariable /[A-Z_-]\+/ contained

" Description node .desc <text>
syn region taskcmdDesc
           \ matchgroup=taskcmdDescTag
           \ start=/desc\%(\s\|$\)/ end=/^\s*\./me=s-1
           \ contained keepend contains=taskcmdInlineCode

" Note node .note <text>
syn region taskcmdNote 
           \ matchgroup=taskcmdNoteTag
           \ start=/note\%(\s\|$\)/ end=/^\s*\./me=s-1
           \ contained keepend contains=taskcmdInlineCode

syn sync fromstart

hi def link taskcmdNode          Keyword
hi def link taskcmdVariable      Constant
hi def link taskcmdDescTag       Keyword
hi def link taskcmdDesc          String
hi def link taskcmdNoteTag       Comment
hi def link taskcmdNote          Comment
hi def link taskcmdInterpolation Constant
hi def link taskcmdInlineCode    Number
hi def link taskcmdPunctuation   Delimiter
hi def link taskcmdDot           Normal

let b:current_syntax = 'taskcmd'

let &cpo = s:cpo_save
unlet s:cpo_save
