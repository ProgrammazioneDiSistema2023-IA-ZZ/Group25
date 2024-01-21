
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Spike {
    /// Stands for "time of the spike", and represents a timestamp of when the spike occurs
    pub spike_time: u128,
    /// Index of the neuron this spike applies to inside its layer
    pub neuron_id: usize,
    /// Index of the layer this spike applies
    pub layer_id: usize
}

impl Spike {
    /// Create a new spike at time `ts` for neuron `neuron_id`
    pub fn new(spike_time: u128, neuron_id: usize, layer_id: usize) -> Spike {
        Spike {
            spike_time,
            neuron_id,
            layer_id
        }
    }

     /// Get the spike time of the current spike
     pub fn get_spike_time(&self) -> u128 {
        self.spike_time
    }

    /*  /// Get the spike neuron id of the current spike
     pub fn get_spike_neuron_id(&self) -> usize {
        self.neuron_id
    }

      /// Get the spike layer id of the current spike
      pub fn get_spike_layer_id(&self) -> usize {
        self.layer_id
    }
 */
    /// Create an array of spikes for a single neuron, given its ID.
    /// The `ts_vec` does not need to be ordered.
    /// 
    
    pub fn create_spike_vec(neuron_id: usize, layer_id: usize, ts_vec: Vec<u128>) -> Vec<Spike> {
        let mut spike_vec : Vec<Spike> = Vec::with_capacity(ts_vec.len());
        
        //Creating the Spikes array for a single Neuron
        for ts in ts_vec.into_iter() {
            spike_vec.push(Spike::new(ts, neuron_id, layer_id));
        }

        //Order the ts vector
        spike_vec.sort();

        spike_vec
    }


    /// Create an ordered array starting from all the spikes sent to the NN.
    /// 
    /// It takes a Matrix where i-th row represents an array of spikes for the i-th entry neuron,
    /// then a single Vec is created. Eventually the array is sorted.
    
    pub fn get_all_spikes(spikes: Vec<Vec<Spike>>) -> Vec<u128> {
        let mut res: Vec<u128> = Vec::new();

        for line in spikes {
            for spike in line {
                res.push(spike.get_spike_time());
            }
        }
        res.sort(); //ascending
    
        res
    }
/* 
    pub fn get_all_spike_time(spikes: Vec<Vec<Spike>>) -> Vec<Vec<u128>> {
        let mut res: Vec<Vec<u128>> = Vec::new();

        for line in spikes {
            let mut riga: Vec<u128> = Vec::new();
            for spike in line {
                riga.push(spike.get_spike_time());
            }
            res.push(riga);
        }
        res.sort(); //ascending
    
        res
    } */
}


pub fn contains_time<'a>(spike_vec: &'a [Spike], time: u128) -> Option<&'a Spike> {
    for spike in spike_vec.iter() {
        if spike.spike_time == time {
            return Some(spike);
        }
    }
    None
}

  // Assumiamo che la struttura Spike sia già definita, ad esempio:
// struct Spike { /* definizione dei campi */ }

pub fn action_spike(spikes: Vec<Vec<Spike>>, time: u128) -> Vec<f64>{

    let mut v = vec![];
    for riga in spikes.iter() {
        match contains_time(&riga, time) {
            Some(_) => {
                v.push(1.0);
            }
            None => {
                v.push(0.0);
            }
        }
    }

    v
}

/* 
pub fn propagate_spike<N: Neuron + 'static>(spike: &mut Spike, network: &NeuralNetwork<N>) {
    let n = spike.neuron_id;
    
    let current_layer_id = spike.layer_id;
    let next_layer_id = spike.layer_id + 1;

    let current_layer = network.get_layer(current_layer_id).expect("Non esiste il layer corrente");

    if let Some(next_layer) = network.get_layer(next_layer_id) {
        for (index, neuron) in next_layer.get_neurons().iter_mut().enumerate() {
            let weight = next_layer.get_input_weight_value(n, index).expect("Non esiste il peso input");
            neuron.put_sum(*weight);
            //println!("Neurone aggiornato: {} {}", index, next_layer_id);
        }
    }

    for (index, neuron) in current_layer.get_neurons().iter_mut().enumerate() {
        let weight = current_layer.get_intra_weight_value(n, index).expect("Non esiste il peso intra");
        neuron.put_sum(*weight);
        //println!("Neurone aggiornato: {} {}", index, current_layer_id);
    }
}



pub fn call_handle_spike<N: Neuron + Clone + 'static>(neuron: &N, time: u128, neuron_id: usize, layer_id: usize, network: &NeuralNetwork<N>) {
    let mut n = neuron.clone(); // Clona il neurone
    println!("Analisi neurone {}-{}", neuron_id, layer_id);
    let result = n.handle_spike(time);
    match result {
        1 => {
            println!("Neurone {}-{} ha sparato al tempo {}", neuron_id, layer_id, time);
            let mut spike = Spike::new(time, neuron_id, layer_id);
            propagate_spike(&mut spike, &network.clone());
        }
        _ => { println!("Neurone {}-{} NON ha sparato al tempo {}", neuron_id, layer_id, time); }
    }
}


pub fn update_neurons<N: Neuron>(time: u128, network: Arc<Mutex<NeuralNetwork<N>>>, network_params: &NeuralNetwork<N>) {
    let network_clone_outer = Arc::clone(&network);
    let mut nn = network_clone_outer.lock().unwrap();

    // Outer loop (layers)
    for (layer_index, layer) in nn.layers.iter_mut().enumerate() {
        // Inner loop (neurons)
        for (neuron_index, neuron) in layer.neurons.iter_mut().enumerate() {
            call_handle_spike(neuron, time, neuron_index, layer_index, network_params);
        }
    }
} */
/* 
pub fn aggiorna_neuroni<N: Neuron>(network: Rc<RefCell<NeuralNetwork<N>>>, ts : f64, spike : Vec<f64>) -> Vec<f64>{
    let mut s = Vec::new();

    let network_clone_inner = Rc::clone(&network);
    let nn = network_clone_inner.borrow_mut();
    
    for i in 0..2{ //for sui layer    
        let temp = Arc::new(Mutex::new(Vec::<f64>::new())); //garantisco la mutua esclusione sul vettore degli spike 
        let mut vt = Vec::new(); //vettore dei thread
        for n in 0..nn.layers.get(i).unwrap().num_neurons(){ //ciclo sui neuroni del layer
            let mut layer_temp = nn.layers.get(i).unwrap().clone();
            let layer_temp_p = nn.layers.get(i-1).unwrap().clone(); //qui abbiamo bisogno anche del layer precedente
            let temp = temp.clone();
            let temporaneo = s.clone();  //spike del layer precedente

            vt.push(std::thread::spawn(move||{
                let mut tot = 0.0; //weighted sum
                for m in 0..layer_temp_p.num_neuroni(){ //ciclo sui neuroni del layer PRECEDENTE
                    tot = tot + temporaneo.get(m).unwrap() * layer_temp.get_interlayer_weight(n,m).unwrap(); //aggiorno il valore dello spike con i pesi del layer precedente
                }
                temp.lock().unwrap().push(layer_temp.get_neuroni_mut(n).unwrap().clone().potential_evolution(tot, ts)); 
            })); //creo un thread per ogni neurone che, calcola la weighted_sum del neurone, aggiorna il potenziale di membrana e ritorna lo spike del Neuron
        }
        for v in vt{
            v.join().unwrap();
        }
        

        //stesso procedimento
        s = temp.lock().unwrap().to_vec();
        drop(temp);
        
        let internal_temp = Arc::new(Mutex::new(vec![0.0; nn.layers.get(i).unwrap().num_neurons()]));
        let mut vet_internal_spike =  Vec::new();
        for n in 0..nn.layers.get(i).unwrap().num_neurons(){
            let internal_temp = internal_temp.clone();
            let layer_temp = nn.layers.get(i).unwrap().clone();
            let temporaneo = s.clone();
            vet_internal_spike.push(std::thread::spawn(move || {
                for m in 0..layer_temp.num_neurons(){                  
                    internal_temp.lock().unwrap()[m] = temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                }
            }));
            
        }
        for v in vet_internal_spike{
            v.join().unwrap();
        }
        
        let lun;
        {
            lun = internal_temp.lock().unwrap().to_vec().len();
        }
        for j in 0..lun{
            self.layers.get(i).unwrap().neuroni.get(j).unwrap().aggiornamento_internal(*internal_temp.lock().unwrap().to_vec().get(j).unwrap());
        }
        drop(internal_temp);

        for indice in 0..s.len(){
            print!("{}", s.get(indice).unwrap());
            print!("  ");
        }
        print!("\n");
    }  
    return s
}       
 */