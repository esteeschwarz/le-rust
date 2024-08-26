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
repl1<-"\\1;\\2;\\3"
#test("\\[(.*?)\\]\\((.*?)\\)(?!#(.+)#) ?((?<=#).+?(?=#))")
t1<-test(t,"\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#)) (#.+#)",repl1)
#tr1<-"\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#)) (#.+#)",repl1)
t1
repl2<-"\\1;\\2;\\3;\\4"
t2<-test(t1,"\\[(.*?)\\]\\((.*?)\\) (.+(?!#(.+)#))",repl2)
t2
repl3<-"\\1;\\2"
t3<-test(t2,"(.*)#note: (.*)#",repl3)
t3
repl4<-"\\1;\\2"
t4<-test(t2,"\\[(.*?)\\]\\((.*?)\\).*",repl4)
t4
t4<-test(t3,"\\[(.*?)\\]\\((.*?)\\).*",repl4)
t4
repl5<-""
t5<-test(t4,";$",repl5)

t5
repl6<-";;;;\\1;;"
t6<-test(t5,"(^[^;].*)",repl6)
t6
