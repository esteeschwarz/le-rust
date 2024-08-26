library(stringi)
t<-readLines("regexcheck.md")
t
regx<-"\\[(.*?)\\]\\((.*?)\\) ?((?!#).+?)?"
test<-function(regx){
stri_extract_all_regex(t,regx)
gsub(regx,"text-\\1--url-\\2--desc-\\3--note-\\4--\\5--\\6--\\7",t,perl = T)
#m
}
test("\\[(.*?)\\]\\((.*?)\\)(?!#(.+)#) ?((?<=#).+?(?=#))")
test("\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#)) (#.+#)")
