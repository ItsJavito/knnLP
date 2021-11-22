#grupo Javier Olazábal - Sebastián Chávarry - Fabrizio figari - Fiorella Valdivia

#funcion normalizar
normalize<-function(val , data , n){
  return((as.numeric(val) - as.numeric(min(data[,n]))) / (as.numeric(max(data[,n]))- as.numeric(min(data[,n]))))
}

#se categoriza data no, sometimes, frequently, always
CategoricData <- function(value) {
  if(value == 'no'){
    return(0)}
  else if(value == 'Sometimes'){
    return(0.3333)
  }else if(value == 'Frequently'){
    return(0.6666)
  }else if(value == 'Always'){
    return(1)
  }else{
    print("error")
    return(-1)
  }
}

#se categoriza data walking,bike, pt, motorbike, automobile
TransData <- function(value) {
  if(value == 'Walking'){
    return(0)}
  else if(value == 'Bike'){
    return(0.25)
  }else if(value == 'Public_Transportation'){
    return(0.5)
  }else if(value == 'Motorbike'){
    return(0.75)
  }else if(value == 'Automobile'){
    return(1)
  }else{
    print("error")
    return(-1)
  }
}



#funcion calcular distancia
calcDist <- function(persona , data){
  #vector donde se va a almacenar la diferencia de la distancia del dataframe con persona
  #y el tipo de peso que le corresponde
  tipo <- c()
  distancia <- c(numeric())
  
  
  for (i in 1:nrow(data)){
    dist = 0
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
    dist = dist + (TransData(persona[16]) - TransData(data[i,16]))^2
    
    dist = sqrt(dist)
    #almacenao la data para dist y el tipo en vectores
    distancia = c(distancia, as.double(dist)) 
    tipo = c(tipo,data[i,17])
    
  }
  distXtipo <- matrix(c(distancia,tipo),nrow = 2111,ncol = 2)
  distXtipo = distXtipo[order(distXtipo[,1]),]
  return(distXtipo)
}

#algoritmo knn
knn <- function(k , distXtipo)
{
  cadena = ""
  index = 1
  max = 0
  flag = FALSE
  
  #vector (usado como dicc) para asociar los tipos de obesidad mas cercanos
  #ejemplo de dibujo de diccionario
  diccTipos <- c("Normal_Weight" = 0 , "Overweight_Level_I" = 0, "Overweight_Level_II" = 0 , "Obesity_Type_III" = 0, 
                 "Obesity_Type_II" = 0 , "Obesity_Type_I" = 0, "Insufficient_Weight" = 0)
  
  
  while (k > 0) 
  {
    diccTipos[distXtipo[index,2]] <- diccTipos[distXtipo[index,2]] + 1
    index = index + 1
    k = k - 1 
    #si ya no quedan m�s vecinos por recorrer
    #cuando el dicc esta completo
    if(k == 0)
    {
      for(i in 1:(length(diccTipos) - 1 ))
      {
        value = diccTipos[i]
        if(max < value){
          cadena = names(value)
          max = value
          flag = FALSE
        }
        else if (max == value){
          flag = TRUE
        }
      }
      #Si la moda se repite en 2 tipos de datos entonces 
      #aumentamos en 1 el k
      if (flag == TRUE)
      {
        k = k + 1
      }
    }
  }
  return(cadena)
}

#leemos el csv
data<-read.csv('data.csv')

persona <- c('Female', 15, 1.7 , 80, 'yes' , 'yes' , 2 , 2 , 'Always' , 'no' , 3 , 'yes', 0, 0, 'no','Walking')

start_time <- Sys.time()
paste("La clasificacion de la persona es:", knn(30,calcDist(persona, data)), sept= " ")
end_time <- Sys.time()

paste("Tiempo de ejecucion :" , (end_time - start_time)*1000 , "ms" , sept = " ")

