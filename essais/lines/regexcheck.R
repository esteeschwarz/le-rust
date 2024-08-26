library(stringi)
t<-readLines("regexcheck.md")
t
#regx<-"\\[(.*?)\\]\\((.*?)\\) ?((?!#).+?)?"
test<-function(t,regx,repl){

  stri_extract_all_regex(t,regx)
t<-gsub(regx,repl,t,perl = T)
#m
t
}
repl1<-"\\1,\\2,\\3"
#test("\\[(.*?)\\]\\((.*?)\\)(?!#(.+)#) ?((?<=#).+?(?=#))")
t1<-test(t,"\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#)) (#.+#)",repl1)
t1
repl2<-"\\1,\\2,,\\3"
t2<-test(t1,"\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#))",repl2)
t2
t3<-test(t2,"\\[(.*?)\\]\\((.*?)\\)(.+)?",repl1)
t3
repl2<-"\\1"
t4<-test(t3,"#note: (.*)#",repl2)
t4
