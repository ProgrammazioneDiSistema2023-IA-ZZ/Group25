# Group25
cose da fare:
-sviluppare la NN
-fare parallelizzazione
-sviluppare il LIF
-sviluppare i comandi per i test sugli errori
-raccogliere i dati sugli errori per studiare la resilienza
-fare presentazione power point

All'interno della NN:
-sviluppare interfaccia neurone di tipo generico ed independete dal modello interno scelto; il neurone riceve impulsi di tipo binario e da in output impulsi di tipo binario
- i parametri interni (potenziale di reset, potenziale di riposo e soglia) devono essere configurabili
- utilizzare struttura fully connected
- numero di strati e neuroni totalmente configurabile


Come sviluppare gli errori:
- considerare una lista di possibili componenti da 'attaccare' (valori di soglie, pesi delle sinapsi, potenziali di membrane)
- per attacco si intende la modifica secondo uno dei 3 errori di uno dei bit di tale valore (-> trasformare il valore in bit ed in seguito eseguire una bit-wise ed impostare il valore ottenuto. es. 5-> 101 bitwise stuck-at-0 sull'ultimo bit ottengo 100 =4)
- possibilità di errori: stuck-at-0, stuck-at-1, transient bit-flip
-indicare il numero di occorrenze di tali guasti ( cioè ripetere i guasti N volte su diversi bit dei diversi componenti da verificare)
- indicare la normale sequenza di input della rete stessa

---- bash ----
git add nomemodulo.rs
git commit -m "???"
git push
---------------



lif_neuron.rs:
- implementiamo il tratto Neuron in cui all'interno è presente un tipo ClassNeuron
- implementiamo una struct di tipo LIFNeuron che contiene i valori (membrane_potential, reset_potential, resting_potential, threshold,tau)
- implementiamo una struct LeakyIntegrateFire
- implementiamo il tratto Neuron per la struct LeakyIntegrateFire in cui il tipo ClassNeuron è LIFNeuron


spike.rs:
- la struttura Spike presenta 3 campi: spike_time per indicare il tempo a cui lo spike fa riferimento, neuron_id per indicare a quale neurone lo spike viene associato, layer_id per indicare a quale layer appartiene tale neurone colpito

- per ogni neurone creaimo un vec di Spike, avendo come input l'id del neurone e il vec degli spike_time. Il vec di Spike viene ordinato in base al vec di spike_time

- la funzione get_all_spikes serve per creare un array ordinato per inserire tutti gli spike mandati alla rete


neural_layer.rs:
- la struct Layer contiene: un vec di neurons, un vec di vec di input weights, per i pesi tra il layer ed il successivo, un vec di vec di intra_weights per indicare i pesi tra neuroni dello stesso layer (contiene solo valori negativi)


neural_network.rs:
- la struct Neural Network contiene un vec di layers

main.rs:
- configuro il neurone di partenza
- configuro la rete
- conneto gli strati
- definisco gli input per la simulazione
- eseguo la simulazione
