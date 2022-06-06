import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'rust.aws-cdk-lambda';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    new RustFunction(this, 'brein_lambda', {
      directory: '../../'
      bin: 'brein_rs',
    });

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'CdkQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
