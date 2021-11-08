normalize<-function(val , data , n){
  return((as.numeric(val) - as.numeric(min(data[,n]))) / (as.numeric(max(data[,n]))- as.numeric(min(data[,n]))))
}

CategoricData <- function(value) {
  if(value == 'no'){
    return(0)
  }
  else if(value == 'Sometimes'){
    return(0.3333)
  }else if(value == 'Frequently'){
    return(0.6666)
  }
  else if(value == 'Always'){
    return(1)
  }else{
    print("error")
    return(-1)
  }
}

calcDist <- function(persona , data){
  res = c()
  dist = 0
  
  for (i in 1:nrow(data)){
    #genero
    if(data[i,1] != persona[1]) dist = dist + 1
    #edad
    dist = dist + (normalize(persona[2], data , 2) - normalize(data[i,2], data, 2))^2
    #altura
    dist = dist + (normalize(persona[3], data , 3) - normalize(data[i,3], data, 3))^2
    #peso
    dist = dist + (normalize(persona[4], data , 4) - normalize(data[i,4], data, 4))^2
    #family history
    if(data[i,5] != persona[5]) dist = dist + 1
    #FAVC 
    if(data[i,6] != persona[6]) dist = dist + 1
    #FCVC 
    dist = dist + (normalize(persona[7], data , 7) - normalize(data[i,7], data, 7))^2
    #NCP 
    dist = dist + (normalize(persona[8], data , 8) - normalize(data[i,8], data, 8))^2
    #caec
    dist = dist + (CategoricData(persona[9]) - CategoricData(data[i,9]))^2
    #smoke
    if(data[i,10] != persona[10]) dist = dist + 1
    #ch20
    dist = dist + (normalize(persona[11], data , 11) - normalize(data[i,11], data, 11))^2
    #SCC
    if(data[i,12] != persona[12]) dist = dist + 1
    #faf
    dist = dist + (normalize(persona[13], data , 13) - normalize(data[i,13], data, 13))^2
    #TUE
    dist = dist + (normalize(persona[14], data , 14) - normalize(data[i,14], data, 14))^2
    #calc
    dist = dist + (CategoricData(persona[15]) - CategoricData(data[i,15]))^2
    #mtrans
    if(data[i,16] != persona[16]) dist = dist + 1
    dist = sqrt(dist)
    res = c(res, c((as.numeric(dist)) , data[i,17]))
    dist = 0
  }
  return(res)
}

#leemos el csv
data<-read.csv('./data.csv')
temp <- nrow(data)
# persona <- rep(NA, 17) <= para cuando hagamos el input
persona = c('Female', 15, 1.7 , 80, 'yes' , 'yes' , 2 , 2 , 'Always' , 'no' , 3 , 'yes', 0, 0, 'no'
            , 'Walking', 'Normal_weight')
var = calcDist(persona, data)
var







