library(stringi)
t<-readLines("regexcheck.md")
t<-readLines("13123.ada.pinghook.md")
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
m<-grep("#####",t5)
t5[m]
library(stringi)
cat<-stri_split_regex   (t5,"##### ",simplify = T)
cat[[1]]
cat
unlist(cat)
cat.array<-c(p=m,cat=strsplit(t5[m],"#####")[2])
cat.array
t5[1]<-"id;h1,linktext;url;note"
#for (k in 1:length(cat[,1])){
  range.s<-m
  range.e<-m-1
  range.s
  
  range.e
  
  range.e<-c(range.e[2:length(range.e)],length(t5))
 #k<-
  for(k in 1:length(range.e)){
    pos<-range.s[k]:range.e[k]
    cat[pos,1]<-cat[range.s[k],2]
  }
  
  cat

#}
#t6<-t5[2:length(t5)]

t6<-paste0(cat[,1],";",t5)
m2<-grepl("#####",t5)
t6<-t6[!m2]
t6<-t6[2:length(t6)]
t6
t6[1]<-"h1;linktext;url;description;note;url.n"
t6<-gsub(",","::",t6)
t.sem<-stri_split_regex(t6,";",simplify = T)
colnames(t.sem)<-t.sem[1,]
t.sem<-t.sem[2:length(t.sem[,1]),]
write.csv(t.sem,"pinghook.r.csv",sep = ",")
t6.md<-knitr::kable(t.sem)

# 
 writeLines(t6.md,"pinghook.out.md")
writeLines(t6.md,"~/Documents/GitHub/pinghook/README.md")
 # repl6<-";;;;\\1;;"
# t6<-test(t5,"(^[^;].*)",repl6)
# t6
# t5
d<-read.csv("pinghook.r.csv",sep = ";")
