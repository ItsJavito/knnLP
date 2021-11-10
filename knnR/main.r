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

crear_persona <- function() 
{
  
  
  while (TRUE) 
  {
    gender <- as.integer(readline(writeLines("Cual es su genero?\nOPCIONES:\n(0) Masculino\n(1) Femenino ")))
    if (gender == 0) {
      gender <- "Male"
      break
    }
    else if (gender == 1) {
      gender <- "Female"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  
  age <- as.integer(readline("Cual es su edad?: \n"))
  
  height <-as.double(readline("Cual es su altura en metros?: \n"))
  
  weight <-as.double(readline("Cual es su peso en kg?: \n"))
  
  
  while (TRUE) 
  {
    family_overweight <-as.integer(readline(writeLines("En su familia alguien sufre o ha sufrido de sobrepeso?\nOPCIONES:\n(0) SI\n(1) NO ")))
    if (family_overweight == 0) {
      family_overweight <- "yes"
      break
    }
    else if (family_overweight == 1) {
      family_overweight <- "no"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  
  
  while (TRUE) 
  {
    favc  <- as.integer(readline(writeLines("Come comida alta en calorias frecuentemente?\nOPCIONES:\n(0) SI\n(1) NO ")))
    if (favc == 0) {
      favc <- "yes"
      break
    }
    else if (favc == 1) {
      favc <- "no"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    fcvc  <-as.integer(readline(writeLines("Come vegetales en su comida?\nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Siempre? ")))
    if (fcvc == 0) 
    {
      fcvc <- 1
      break
    }
    else if (fcvc == 1) {
      fcvc <- 2
      break
    }
    else if (fcvc == 2) {
      fcvc <- 3
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    ncp  <-as.integer(readline(writeLines("Cuantas comidas principales ingiere en el dia?\nOPCIONES:\nINgrese un n�mero entre 1-4")))
    if (ncp >= 1 | ncp <=4){ 
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    caec  <- as.integer(readline(writeLines("Come algo durante sus comidas? \nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Frecuentemente\n(3) Siempre ")))
    if (caec == 0) {
      caec <- "no"
      break
    }
    else if (caec == 1) {
      caec <- "Sometimes"
      break
    }
    else if (caec == 2) {
      caec <- "Frequently"
      break
    }
    else if (caec == 3) {
      caec <- "Always"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    smoke <- as.integer(readline(writeLines("Usted fuma?\n OPCIONES:\n(0) SI\n(1) NO ")))
    if (smoke == 0) {
      smoke <- "yes"
      break
    }
    else if (smoke == 1) {
      smoke <- "no"
      break
    }
    
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  
  while (TRUE) 
  {
    ch20 <-as.integer(readline(writeLines("Cuanta agua bebe al dia?\nOPCIONES:\n(0) Menos de 1 litro\n(1) Entre 1 y 2 litros\n(2) Mas de 2 litros ")))
    
    if (ch20 == 0) 
    {
      ch20 <- 1
      break
    }
    else if (ch20 == 1) {
      ch20 <- 2
      break
    }
    else if (ch20 == 2) {
      ch20 <- 3
      break
    }
    
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    scc <-as.integer(readline(writeLines("Monitorea sus calorias diariamente?\nOPCIONES:\n(0) SI\n(1) NO ")))
    if (scc == 0) {
      scc <- "yes"
      break
    }
    else if (scc == 1) {
      scc <- "no"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    faf <-as.integer(readline(writeLines("Cuantas veces a la semana hace actividad fisica?\nOPCIONES:\n(0) Nunca\n(1) 1 o 2 dias\n(2) 2 o 4 dias\n(3) 4 o 5 dias ")))
    if (faf == 0 | faf == 1 | faf== 2 | faf== 3) {
      break
    }
    
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  
  while (TRUE) 
  {
    tue <-as.integer(readline(writeLines("Cuanto tiempo usa aparatos tecnologicos?\nOPCIONES:\n(0) 0 a 2 horas\n(1) 3 a 5 horas\n(2) Mas de 5 horas ")))
    if (tue == 0 | tue == 1 | tue== 2) {
      break
    }
    
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  while (TRUE) 
  {
    calc <-as.integer(readline(writeLines("Que tan amenudo ingiere alcohol?\nOPCIONES:\n(0) Nunca\n(1) A veces\n(2) Frecuentemente\n(3) Siempre ")))
    if (calc == 0) {
      calc <- "no"
      break
    }
    else if (calc == 1) {
      calc <- "Sometimes"
      break
    }
    else if (calc == 2) {
      calc <- "Frequently"
      break
    }
    else if (calc == 3) {
      calc <- "Always"
      break
    }
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  
  
  
  while (TRUE) 
  {
    mtran <-as.integer(readline(writeLines("Que medio de transporte normalmente usa?\nOPCIONES:\n(0) Automovil\n(1) Motocileta\n(2) Bicicleta\n(3) Transporte publico\n(4) Caminando ")))
    if (mtran == 0) {
      mtran <- "Automobile"
      break
    }
    else if (mtran == 1) {
      mtran <- "Motorbike"
      break
    }
    else if (mtran == 2) {
      mtran <- "Bike"
      break
    }
    else if (mtran == 3) {
      mtran <- "Public_Transportation"
      break
    }
    else if (mtran == 4) {
      mtran <- "Walking"
      break
    }
    
    else
    {
      print("Ingresar un valor dentro de las opciones")    
    }
  }
  
  persona = c(gender,age,height,weight,family_overweight,favc,fcvc,ncp,caec,smoke,ch20,scc,faf,tue,calc,mtran)
  
  return(persona)
  
}



#leemos el csv
data<-read.csv('data.csv')

# persona <- rep(NA, 17) <= para cuando hagamos el input
persona <- crear_persona()
#print(persona)

#persona2 <- c('Female', 15, 1.7 , 80, 'yes' , 'yes' , 2 , 2 , 'Always' , 'no' , 3 , 'yes', 0, 0, 'no','Walking')

paste("La clasificacion de la persona es:", knn(30,calcDist(persona, data)), sept= " ")

