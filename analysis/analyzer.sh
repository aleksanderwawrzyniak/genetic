#!/usr/bin/env bash


# set constant values
POPULATION=100
CROSSOVER_RATE=0.85
CUTTING_POINT=20
MUTATION_RATE=0.003
ITERATIONS=500
REPEATS=2
TOURNAMENT_SIZE=$((POPULATION * 25 / 100))

RUN='../target/release/genetic'
TASKS='../tasks.csv'

avg_dir='results/average'
iter_dir='results/iterations'
plots_dir='plots'
result_file='results.txt'

POPULATION_SIZES=(10, 25, 50, 100, 250, 500)
CROSSOVER_RATES=(0.2, 0.4, 0.6, 0.7, 0.85, 0.9, 1.0)
MUTATION_RATES=(0.001, 0.003, 0.005, 0.01, 0.02, 0.03)
TOURNAMENT_SIZES=(10, 20, 25, 50, 75, 90)



rm -rf results 2> /dev/null || echo -n
mkdir -p ${iter_dir} 2> /dev/null
mkdir -p ${avg_dir} 2> /dev/null
mkdir ${plots}

for population_size in "${POPULATION_SIZES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $RUN evolve --input $TASKS \
        --population-size $population_size \
        --crossover-rate $CROSSOVER_RATE \
        --cutting-point $CUTTING_POINT \
        --iterations $ITERATIONS
        --mutation-rate $MUTATION_RATE
        --tournament-size $(($population_size * 25 / 100)) \
        && mv ${result_file} ${iter_dir}/res_${iteration}.txt
        
    done
    
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py ${iter_files} \
    1> ${avg_dir}/${population_size}.txt 2> /dev/null
    rm ${iter_files} 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/*))
python plotter.py "population size" ${plots_dir}/population_size.jpg ${files}
rm ${files}

for crossover_rate in "${CROSSOVER_RATES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $RUN evolve --input $TASKS \
        --population-size $POPULATION \
        --crossover-rate $crossover_rate \
        --cutting-point $CUTTING_POINT \
        --iterations $ITERATIONS
        --mutation-rate $MUTATION_RATE
        --tournament-size $TOURNAMENT_SIZE \
        && mv ${result_file} ${iter_dir}/res_${iteration}.txt
        
    done
    
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py ${iter_files} \
    1> ${avg_dir}/${crossover_rate}.txt 2> /dev/null
    rm ${iter_files} 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/*))
python plotter.py "crossover rate" ${plots_dir}/crossover_rate.jpg ${files}
rm ${files}

for mutation_rate in "${MUTATION_RATES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $RUN evolve --input $TASKS \
        --population-size $POPULATION \
        --crossover-rate $CROSSOVER_RATE \
        --cutting-point $CUTTING_POINT \
        --iterations $ITERATIONS
        --mutation-rate $mutation_rate
        --tournament-size $TOURNAMENT_SIZE \
        && mv ${result_file} ${iter_dir}/res_${iteration}.txt
        
    done
    
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py ${iter_files} \
    1> ${avg_dir}/${mutation_rate}.txt 2> /dev/null
    rm ${iter_files} 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/*))
python plotter.py "mutation rate" ${plots_dir}/mutation_rate.jpg ${files}
rm ${files}

for tournament_size in "${TOURNAMENT_SIZES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $RUN evolve --input $TASKS \
        --population-size $POPULATION \
        --crossover-rate $CROSSOVER_RATE \
        --cutting-point $CUTTING_POINT \
        --iterations $ITERATIONS
        --mutation-rate $MUTATION_RATE
        --tournament-size $tournament_size \
        && mv ${result_file} ${iter_dir}/res_${iteration}.txt
        
    done
    
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py ${iter_files} \
    1> ${avg_dir}/${tournament_size}.txt 2> /dev/null
    rm ${iter_files} 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/*))
python plotter.py "tournament size" ${plots_dir}/tournament_size.jpg ${files}
rm ${files}