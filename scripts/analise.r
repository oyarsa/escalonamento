  library(plyr)
  
  le.arquivos <- function(caminho) {
    arquivos <- list.files(caminho, pattern = ".*.csv")
    ldply(arquivos, .fun = function(x) {
      read.csv(file=x, stringsAsFactors=FALSE)
    })
  }
  
  join.nl <- function(...) {
    paste(list(...), collapse='\n')
  }
  
  rpd <- function(f.method, f.best) {
    abs((f.method - f.best)/f.best) * 100 
  }
  
  rpd.reduce <- function(f, col, f.best) {
    f(sapply(col, function(cur) rpd(cur, f.best)))
  }
  
  frames <- le.arquivos('.')
  result <- ddply(frames, ~ID, summarise, 
                  Media.FO=mean(FO), Mediana.FO=median(FO), Max.FO=max(FO), Min.FO=min(FO),
                  Media.Tempo=mean(Tempo), Mediana.Tempo=median(Tempo), Min.Tempo=min(Tempo), Max.Tempo=max(Tempo),
                  Media.RPD.FO=(function(x) { rpd.reduce(mean, x, min(frames$FO)) })(FO),
                  Mediana.RPD.FO=(function(x) { rpd.reduce(median, x, min(frames$FO)) })(FO),
                  RPD.Tempo=(function(x) { rpd.reduce(mean, x, min(frames$Tempo)) })(Tempo),
                  DesvioPadrao.FO=sd(FO),
                  Media.IterAlvo=mean(IterAlvo),
                  Min.IterAlvo=min(IterAlvo),
                  Max.IterAlvo=max(IterAlvo),
                  Mediana.IterAlvo=median(IterAlvo),
                  Media.TotalIter=mean(TotalIter),
                  Mediana.TotalIter=median(TotalIter),
                  Min.TotalIter=min(TotalIter),
                  Max.TotalIter=max(TotalIter)
                  )
  
  result <- result[with(result, order(-result$Mediana.FO)), ]
  res10 <- head(result, 10)
  dp <- res10[with(res10, order(res10$DesvioPadrao.FO)), ]