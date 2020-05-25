#!/usr/bin/env bash


set -e

LANG=en_US
LC_NUMERIC=en_US.UTF-8

# set constant values
POPULATION=1000
CROSSOVER_RATE=0.85
CUTTING_POINT=200
MUTATION_RATE=0.003
ITERATIONS=750
REPEATS=10
DENSITY=8
TOURNAMENT_SIZE=$((POPULATION * 25 / 100))

BINARY='../target/release/genetic'
TASKS='../tasks.csv'

avg_dir=results/average
iter_dir=results/iterations
plots_dir=plots
result_file=results.txt

POPULATION_SIZES=(10 25 50 100 250 500 1000)
CROSSOVER_RATES=(0.2 0.4 0.6 0.7 0.85 0.9 1.0)
MUTATION_RATES=(0.001 0.002 0.003 0.004 0.005 0.01 0.02)
TOURNAMENT_SIZES=(10 20 25 50 75 90 100)
DENSITIES=(2 4 8 10 20 50 100)
CUTTING_POINTS=(10 50 100 250 500 750 1000 1250)



# I do nor generate new set every time the script is BINARY,
# so the output can be somewhat reproductible
rm -rf results 2> /dev/null || echo -n
rm -rf ${plots_dir} 2> /dev/null || echo -n
mkdir -p ${iter_dir} 2> /dev/null
mkdir -p ${avg_dir} 2> /dev/null
mkdir -p ${plots_dir} 2> /dev/null

for population_size in "${POPULATION_SIZES[@]}"; do
    for iteration in $(seq 1 $REPEATS); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $population_size \
        --crossover-rate    $CROSSOVER_RATE \
        --cutting-point     $CUTTING_POINT \
        --iterations        $ITERATIONS \
        --mutation-rate     $MUTATION_RATE \
        --tournament-size   $((population_size * 25 / 100)) \
        --density           $DENSITY \
        --output            ${iter_dir}/res_${iteration}.txt \
        --random-cutting-point \
        &> /dev/null
        
    done
    
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/${population_size}.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting population size"
python plotter.py "population size" ${plots_dir}/population_size.jpg "${files[@]}"
rm "${files[@]}"

for crossover_rate in "${CROSSOVER_RATES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $POPULATION \
        --crossover-rate    $crossover_rate \
        --cutting-point     $CUTTING_POINT \
        --iterations        $ITERATIONS \
        --mutation-rate     $MUTATION_RATE \
        --tournament-size   $TOURNAMENT_SIZE \
        --density           $DENSITY \
        --output            ${iter_dir}/res_${iteration}.txt \
        --random-cutting-point \
        &> /dev/null
        
    done
    
    sleep 1
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/${crossover_rate}.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting crossover rate"
python plotter.py "crossover rate" ${plots_dir}/crossover_rate.jpg "${files[@]}"
rm "${files[@]}"

for mutation_rate in "${MUTATION_RATES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $POPULATION \
        --crossover-rate    $CROSSOVER_RATE \
        --cutting-point     $CUTTING_POINT \
        --iterations        $ITERATIONS \
        --mutation-rate     $mutation_rate \
        --tournament-size   $TOURNAMENT_SIZE \
        --density           $DENSITY \
        --output            ${iter_dir}/res_${iteration}.txt \
        --random-cutting-point \
        &> /dev/null
        
    done
    
    sleep 1
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/${mutation_rate}.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting mutation rate"
python plotter.py "mutation rate" ${plots_dir}/mutation_rate.jpg "${files[@]}"
rm "${files[@]}"

for tournament_size in "${TOURNAMENT_SIZES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $POPULATION \
        --crossover-rate    $CROSSOVER_RATE \
        --cutting-point     $CUTTING_POINT \
        --iterations        $ITERATIONS \
        --mutation-rate     $MUTATION_RATE \
        --tournament-size   $tournament_size \
        --density           $DENSITY \
        --output            ${iter_dir}/res_${iteration}.txt \
        --random-cutting-point \
        &> /dev/null
        
    done
    
    sleep 1
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/${tournament_size}.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting tournament size"
python plotter.py "tournament size" ${plots_dir}/torunament_size.jpg "${files[@]}"
rm "${files[@]}"

for density in "${DENSITIES[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $POPULATION \
        --crossover-rate    $CROSSOVER_RATE \
        --cutting-point     $CUTTING_POINT \
        --iterations        $ITERATIONS \
        --mutation-rate     $MUTATION_RATE \
        --tournament-size   $TOURNAMENT_SIZE \
        --density           $density \
        --output            ${iter_dir}/res_${iteration}.txt \
        --random-cutting-point \
        &> /dev/null
        
    done
    
    sleep 1
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/$((100 / density))%.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting density"
python plotter.py "density" ${plots_dir}/density.jpg "${files[@]}"
rm "${files[@]}"

for cutting_point in "${CUTTING_POINTS[@]}"; do
    for iteration in $(seq 1 ${REPEATS}); do
        
        $BINARY evolve --input $TASKS \
        --population-size   $POPULATION \
        --crossover-rate    $CROSSOVER_RATE \
        --cutting-point     $cutting_point \
        --iterations        $ITERATIONS \
        --mutation-rate     $MUTATION_RATE \
        --tournament-size   $TOURNAMENT_SIZE \
        --density           $density \
        --output            ${iter_dir}/res_${iteration}.txt \
        --try-recover \
        &> /dev/null
        
    done
    
    sleep 1
    iter_files=($(ls -d ${iter_dir}/* 2> /dev/null))
    python averager.py "${iter_files[@]}" \
    1> ${avg_dir}/${cutting_point}.txt 2> /dev/null
    rm "${iter_files[@]}" 2> /dev/null
    
done

files=($(ls -d ${avg_dir}/* 2> /dev/null))
echo "plotting cutting point"
python plotter.py "cutting point" ${plots_dir}/cutting_point.jpg "${files[@]}"
rm "${files[@]}"
