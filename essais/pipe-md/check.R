t<-readtext::readtext("sample.md")$text
re_h1<-"# ?(.+?)<"
re_h2 = "## (.+?)<"
re_h3 = "### (.+?)<"
re_h4 = "#### (.+?)<"
re_h5 = "##### (.+?)<"
re_h6 = "###### (.+?)<"
re_p ="\n(.+?)\n|\n(.+?)$"
#re_p ="\n(.+?)(\n)"
t1<-gsub(re_p,"<p>\\1\\2</p>",t)
t
t1
t1<-gsub(re_h6,"<h6>\\1</h6>",t1)
t1<-gsub(re_h5,"<h5>\\1</h5>",t1)
t1<-gsub(re_h4,"<h4>\\1</h4>",t1)
t1<-gsub(re_h3,"<h3>\\1</h3>",t1)
t1<-gsub(re_h2,"<h2>\\1</h2>",t1)
t1<-gsub(re_h1,"<h1>\\1</h1>",t1)
t1
