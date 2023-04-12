use std::{collections::HashMap, fs};

use serde::Serialize;

/// A collection of network-related stats

#[derive(Debug, Serialize)]
pub struct NetworkInterfaceStats {
    rx_bytes: u64,
    rx_packets: u64,
    rx_errors: u64,
    rx_dropped_or_missed: u64,
    rx_fifo_errors: u64,
    rx_frame_errors: u64, // rx_length_errors + rx_over_errors + rx_crc_errors + rx_frame_errors
    rx_compressed: u64,
    multicast: u64,
    tx_bytes: u64,
    tx_packets: u64,
    tx_errors: u64,
    tx_dropped: u64,
    tx_fifo_errors: u64,
    collisions: u64,
    tx_carrier_errors: u64, // carrier_errors + tx_aborted_errors + tx_window_errors + tx_heartbeat_errors
    tx_compressed: u64,
}

pub fn network_interface_stats() -> HashMap<String, NetworkInterfaceStats> {
    // The contents of `/proc/net/dev` is fairly hideous:
    // $ cat /proc/net/dev
    // Inter-|   Receive                                                |  Transmit
    // face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    //    lo:     476       4    0    0    0     0          0         0      476       4    0    0    0     0       0          0
    //  eth0: 201837803305 164530931    0 80597    0     0          0         0 17291765307 97844243    0    0    0     0       0          0
    //
    // ...and so on.
    //
    // Hat tip to this Stack Overflow answer https://stackoverflow.com/a/4943975 for pointing out
    // where in the Linux source code these fields came from; the code has moved since that answer
    // was written and is now here:
    // https://github.com/torvalds/linux/blob/38e04b3/net/core/net-procfs.c#L77-L99

    fs::read_to_string("/proc/net/dev")
        .expect("Failed to read /proc/net/dev")
        .trim()
        .split('\n')
        .filter_map(|line| {
            line.split_once(':').map(|(interface_name, stats)| {
                let stats = stats
                    .split_whitespace()
                    .map(|stat| stat.parse().unwrap())
                    .collect::<Vec<u64>>();
                assert_eq!(stats.len(), 16);

                (
                    interface_name.trim().to_string(),
                    NetworkInterfaceStats {
                        rx_bytes: stats[0],
                        rx_packets: stats[1],
                        rx_errors: stats[2],
                        rx_dropped_or_missed: stats[3],
                        rx_fifo_errors: stats[4],
                        rx_frame_errors: stats[5],
                        rx_compressed: stats[6],
                        multicast: stats[7],
                        tx_bytes: stats[8],
                        tx_packets: stats[9],
                        tx_errors: stats[10],
                        tx_dropped: stats[11],
                        tx_fifo_errors: stats[12],
                        collisions: stats[13],
                        tx_carrier_errors: stats[14],
                        tx_compressed: stats[15],
                    },
                )
            })
        })
        .collect()
}
