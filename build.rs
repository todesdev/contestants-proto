/*
 * Copyright (c) 2024 Vladimirs Duhovs <vlad.duhovs@gmail.com>. All rights reserved.
 *
 * The software and associated documentation files (the "Software") are proprietary and confidential.
 *
 * Unauthorized copying, sharing, distribution, reproduction, or any form of publication of the Software,
 * either in whole or in part, is strictly prohibited without prior written consent from Vladimirs Duhovs.
 *                                                                                                  
 * This Software is provided by the copyright holder "as is" and any express or implied warranties,
 * including, but not limited to, the implied warranties of merchantability and fitness for a particular purpose are disclaimed.
 *
 * In no event shall Vladimirs Duhovs be liable for any direct, indirect, incidental, special, exemplary, or consequential damages
 * (including, but not limited to, procurement of substitute goods or services; loss of use, data, or profits; or business interruption)
 * however caused and on any theory of liability, whether in contract, strict liability, or tort (including negligence or otherwise)
 * arising in any way out of the use of this Software, even if advised of the possibility of such damage.
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/contestants.proto")?;
    Ok(())
}